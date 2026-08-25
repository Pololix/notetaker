use crate::{
    buffer::{Buffer, BufferId},
    event::workspace_event::{WorkspaceCommand, WorkspaceEvent},
};
use editor_common::{
    color::Color,
    geometry::{Point, Rect, Viewport},
    rendering::RenderCommand,
};
use std::collections::HashMap;

type NodeId = usize;
pub type WorkspaceId = usize;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum WorkspaceError {
    #[error("Failed to retrieve a root node")]
    NullRoot,

    #[error("Failed to retrieve the active node")]
    NullActive,

    #[error("Failed to retrieve a node from the given id")]
    InvalidNodeId,

    #[error("Failed to retrieve a buffer from the given id")]
    InvalidBufferId,

    #[error("Tried to perform a workspace command with a split node bound as active")]
    InvalidCaller,

    #[error("The tree has an invalid structure")]
    InvalidTree(String),
}

#[derive(Debug, Clone, Copy)]
pub enum SplitMode {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
struct Node {
    pub parent_id: Option<NodeId>,
    pub ty: NodeType,
}

#[derive(Debug, Clone)]
enum NodeType {
    Split {
        area: Rect,
        mode: SplitMode,
        first_id: NodeId,
        second_id: NodeId,
    },
    Buffer {
        buffer_id: BufferId,
        surface: Rect,
        cursor: usize,
        v_scroll: u32,
        h_scroll: u32,
    },
}

#[derive(Debug)]
pub struct Workspace {
    nodes: HashMap<NodeId, Node>,
    active_id: Option<NodeId>,
    next_node_id: NodeId,

    buffers: HashMap<BufferId, Buffer>,
    next_buff_id: BufferId,
}

impl Workspace {
    pub fn new(viewport: Viewport) -> Result<Self, WorkspaceError> {
        let mut new_self = Self {
            nodes: HashMap::new(),
            next_node_id: 0,
            buffers: HashMap::new(),
            next_buff_id: 0,
            active_id: None,
        };

        let _event = new_self.add_buffer(viewport)?;

        Ok(new_self)
    }

    pub fn adapt_to_viewport(&mut self, viewport: Viewport) -> Result<(), WorkspaceError> {
        let root_id = match self.nodes.iter().find(|(_, node)| node.parent_id.is_none()) {
            Some((id, _)) => *id,
            None => return Err(WorkspaceError::NullRoot), // possible if empty
        };

        self.restore_geometry_recursive(
            root_id,
            Rect {
                coords: Point { x: 0.0, y: 0.0 },
                width: viewport.width as f32,
                height: viewport.height as f32,
            },
        )?;

        Ok(())
    }

    pub fn render(&self) -> Vec<RenderCommand> {
        self.nodes
            .values()
            .filter_map(|node| match node.ty {
                NodeType::Buffer {
                    buffer_id,
                    surface,
                    cursor,
                    v_scroll,
                    h_scroll,
                } => {
                    let text = self.get_buffer(buffer_id).ok()?.get_text();
                    let color = Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    };

                    Some(RenderCommand::Text {
                        surface,
                        text,
                        color,
                    })
                }

                NodeType::Split { .. } => None,
            })
            .collect()
    }

    pub fn handle_command(
        &mut self,
        cmd: WorkspaceCommand,
    ) -> Result<Option<WorkspaceEvent>, WorkspaceError> {
        let event = match cmd {
            WorkspaceCommand::OpenBuffer { viewport } => match self.active_id {
                Some(_) => Some(self.split_active(SplitMode::Vertical)?),
                None => Some(self.add_buffer(viewport)?),
            },
            WorkspaceCommand::SplitBuffer { mode } => Some(self.split_active(mode)?),
            WorkspaceCommand::QuitBuffer => Some(self.quit_active()?),
            _ => None,
        };

        Ok(event)
    }

    fn add_buffer(&mut self, viewport: Viewport) -> Result<WorkspaceEvent, WorkspaceError> {
        // create a new buffer node occupying the given rect
        let buffer_id = self.next_buff_id;
        self.next_buff_id += 1;
        let buffer = Buffer::new();

        let node_id = self.next_node_id;
        self.next_node_id += 1;
        let node = Node {
            parent_id: None,
            ty: NodeType::Buffer {
                buffer_id,
                surface: Rect {
                    coords: Point { x: 0.0, y: 0.0 },
                    width: viewport.width as f32,
                    height: viewport.height as f32,
                },
                cursor: 0,
                v_scroll: 0,
                h_scroll: 0,
            },
        };
        self.active_id = Some(node_id);

        self.buffers.insert(buffer_id, buffer);
        self.nodes.insert(node_id, node);

        Ok(WorkspaceEvent::BufferOpened)
    }

    fn split_active(&mut self, mode: SplitMode) -> Result<WorkspaceEvent, WorkspaceError> {
        let (active, active_id) = match self.active_id {
            Some(id) => (self.get_node(id)?.clone(), id),
            None => return Err(WorkspaceError::NullActive),
        };

        let (buffer_id, surface, cursor, v_scroll, h_scroll) = match active.ty {
            NodeType::Buffer {
                buffer_id,
                surface,
                cursor,
                v_scroll,
                h_scroll,
            } => (buffer_id, surface, cursor, v_scroll, h_scroll),
            NodeType::Split { .. } => return Err(WorkspaceError::InvalidCaller),
        };

        // promote active node to a split node
        let first_id = self.next_node_id;
        self.next_node_id += 1;
        let second_id = self.next_node_id;
        self.next_node_id += 1;

        self.get_mut_node(active_id)?.ty = NodeType::Split {
            mode,
            first_id,
            second_id,
            area: surface,
        };

        // create children from the splitted surface
        let (first_surface, second_surface) = Self::split_rect(surface, mode);
        let first_child = Node {
            parent_id: Some(active_id),
            ty: NodeType::Buffer {
                buffer_id,
                surface: first_surface,
                cursor,
                v_scroll,
                h_scroll,
            },
        };
        let second_child = Node {
            parent_id: Some(active_id),
            ty: NodeType::Buffer {
                buffer_id,
                surface: second_surface,
                cursor,
                v_scroll,
                h_scroll,
            },
        };

        self.active_id = Some(second_id);
        self.nodes.insert(first_id, first_child);
        self.nodes.insert(second_id, second_child);

        Ok(WorkspaceEvent::BufferOpened)
    }

    fn quit_active(&mut self) -> Result<WorkspaceEvent, WorkspaceError> {
        let (active, active_id) = match self.active_id {
            Some(id) => (self.get_node(id)?.clone(), id),
            None => return Err(WorkspaceError::NullActive),
        };

        // return error if calling from a split node
        if let NodeType::Split { .. } = active.ty {
            return Err(WorkspaceError::InvalidCaller);
        };

        // remove active from nodes if root
        let (parent, parent_id) = match active.parent_id {
            Some(id) => (self.get_node(id)?.clone(), id),
            None => {
                self.nodes.remove(&active_id);
                self.active_id = None;
                return Ok(WorkspaceEvent::BufferQuit);
            }
        };

        let (area, sibling_id) = match parent.ty {
            NodeType::Split {
                area,
                first_id,
                second_id,
                ..
            } => {
                let id = if active_id == first_id {
                    second_id
                } else if active_id == second_id {
                    first_id
                } else {
                    return Err(WorkspaceError::InvalidTree(
                        "Node references an incoherent parent".to_string(),
                    ));
                };

                (area, id)
            }
            NodeType::Buffer { .. } => {
                return Err(WorkspaceError::InvalidTree(
                    "Buffer node has a buffer node parent".to_string(),
                ));
            }
        };

        // update siblings children's parent_id if any
        let sibling = self.get_node(sibling_id)?.clone();
        match sibling.ty {
            NodeType::Split {
                first_id,
                second_id,
                ..
            } => {
                self.get_mut_node(first_id)?.parent_id = Some(parent_id);
                self.get_mut_node(second_id)?.parent_id = Some(parent_id);
            }
            NodeType::Buffer { .. } => {}
        }

        // promote parent to whatever type sibling is
        self.get_mut_node(parent_id)?.ty = sibling.ty;
        self.restore_geometry_recursive(parent_id, area)?;

        // remove now stale nodes
        self.nodes.remove(&active_id);
        self.nodes.remove(&sibling_id);

        self.set_active_recursive(parent_id)?;

        Ok(WorkspaceEvent::BufferQuit)
    }

    fn restore_geometry_recursive(
        &mut self,
        id: NodeId,
        free_area: Rect,
    ) -> Result<(), WorkspaceError> {
        let node = self.get_mut_node(id)?.clone();

        match node.ty {
            // split area and call recursively
            NodeType::Split {
                mode,
                first_id,
                second_id,
                ..
            } => {
                let (first_area, second_area) = Self::split_rect(free_area, mode);
                self.restore_geometry_recursive(first_id, first_area)?;
                self.restore_geometry_recursive(second_id, second_area)?;
            }
            // consume directly if it is a buffer node
            NodeType::Buffer { .. } => {
                if let NodeType::Buffer { surface, .. } = &mut self.get_mut_node(id)?.ty {
                    *surface = free_area;
                }
                return Ok(());
            }
        }

        Ok(())
    }

    fn set_active_recursive(&mut self, id: NodeId) -> Result<bool, WorkspaceError> {
        match self.get_node(id)?.ty.clone() {
            // call recursively
            NodeType::Split {
                first_id,
                second_id,
                ..
            } => {
                if self.set_active_recursive(first_id)? {
                    Ok(true)
                } else if self.set_active_recursive(second_id)? {
                    Ok(true)
                } else {
                    Err(WorkspaceError::InvalidTree(
                        "Split node has no buffer children".to_string(),
                    ))
                }
            }
            // end at the first buffer
            NodeType::Buffer { .. } => {
                self.active_id = Some(id);
                return Ok(true);
            }
        }
    }

    fn split_rect(rect: Rect, mode: SplitMode) -> (Rect, Rect) {
        match mode {
            SplitMode::Vertical => {
                (
                    // left rect
                    Rect {
                        coords: Point {
                            x: rect.coords.x,
                            y: rect.coords.y,
                        },
                        width: rect.width * 0.5,
                        height: rect.height,
                    },
                    // right rect (offset in x axis)
                    Rect {
                        coords: Point {
                            x: rect.coords.x + rect.width * 0.5,
                            y: rect.coords.y,
                        },
                        width: rect.width * 0.5,
                        height: rect.height,
                    },
                )
            }
            SplitMode::Horizontal => {
                (
                    // top rect
                    Rect {
                        coords: Point {
                            x: rect.coords.x,
                            y: rect.coords.y,
                        },
                        width: rect.width,
                        height: rect.height * 0.5,
                    },
                    // bottom rect (offset in y axis)
                    Rect {
                        coords: Point {
                            x: rect.coords.x,
                            y: rect.coords.y + rect.height * 0.5,
                        },
                        width: rect.width,
                        height: rect.height * 0.5,
                    },
                )
            }
        }
    }

    fn get_node(&self, id: NodeId) -> Result<&Node, WorkspaceError> {
        match self.nodes.get(&id) {
            Some(node) => Ok(node),
            None => return Err(WorkspaceError::InvalidNodeId),
        }
    }

    fn get_mut_node(&mut self, id: NodeId) -> Result<&mut Node, WorkspaceError> {
        match self.nodes.get_mut(&id) {
            Some(node) => Ok(node),
            None => return Err(WorkspaceError::InvalidNodeId),
        }
    }

    fn get_buffer(&self, id: BufferId) -> Result<&Buffer, WorkspaceError> {
        match self.buffers.get(&id) {
            Some(buffer) => Ok(buffer),
            None => return Err(WorkspaceError::InvalidBufferId),
        }
    }

    fn get_mut_buffer(&mut self, id: BufferId) -> Result<&mut Buffer, WorkspaceError> {
        match self.buffers.get_mut(&id) {
            Some(buffer) => Ok(buffer),
            None => return Err(WorkspaceError::InvalidBufferId),
        }
    }
}
