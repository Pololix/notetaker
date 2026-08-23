// TODO
// - add padding between buffers and margins
// - make API config-driven
// - modify to allow resizing buffer surfaces
// - movement between buffers (only view nodes)

use crate::{
    buffer::{Buffer, BufferId, BufferView},
    event::workspace_event::WorkspaceCommand,
};
use editor_common::{Rect, Viewport};

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
    pub id: NodeId,
    parent_id: Option<NodeId>,
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
        v_scroll: f32,
        h_scroll: f32,
    },
}

#[derive(Debug)]
pub struct Workspace {
    pub id: WorkspaceId,

    active_id: Option<NodeId>,
    nodes: Vec<Node>,
    next_node_id: NodeId,

    buffers: Vec<Buffer>,
    next_buff_id: BufferId,
}

#[derive(Debug)]
pub struct WorkspaceView {
    pub buffer_views: Vec<BufferView>,
}

impl Workspace {
    pub fn new(id: WorkspaceId, viewport: Viewport) -> Self {
        let mut new_self = Self {
            id,
            nodes: vec![],
            next_node_id: 0,
            buffers: vec![],
            next_buff_id: 0,
            active_id: None,
        };

        new_self.add_buffer(Rect {
            x: 0.0,
            y: 0.0,
            width: viewport.width as f32,
            height: viewport.height as f32,
        });

        new_self
    }

    pub fn adapt_to_viewport(&mut self, viewport: Viewport) -> Result<(), WorkspaceError> {
        let root_id = match self.nodes.iter().find(|node| node.parent_id.is_none()) {
            Some(node) => node.id,
            None => return Err(WorkspaceError::NullRoot),
        };
        self.restore_geometry_recursive(
            root_id,
            Rect {
                x: 0.0,
                y: 0.0,
                width: viewport.width as f32,
                height: viewport.height as f32,
            },
        )?;

        Ok(())
    }

    // pub fn get_view(&self) -> Result<WorkspaceView, WorkspaceError> {}

    // pub fn handle_command(&mut self, cmd: WorkspaceCommand) Result<WorkspaceEvent,WorkspaceError> {}

    fn add_buffer(&mut self, surface: Rect) {
        // create a new buffer node occupying the given rect
        let buffer_id = self.next_buff_id;
        self.next_buff_id += 1;
        let buffer = Buffer::new(buffer_id);

        let node = Node {
            id: self.next_node_id,
            parent_id: None,
            ty: NodeType::Buffer {
                buffer_id,
                surface,
                cursor: 0,
                v_scroll: 0.0,
                h_scroll: 0.0,
            },
        };
        self.next_node_id += 1;
        self.active_id = Some(node.id);

        self.buffers.push(buffer);
        self.nodes.push(node);
    }

    fn split_active(&mut self, mode: SplitMode) -> Result<(), WorkspaceError> {
        let active_id = match self.active_id {
            Some(id) => id,
            None => return Err(WorkspaceError::NullActive),
        };
        let active_index = self.get_node_index(active_id)?;

        // fetch current buffer surface and id
        let (buffer_id, surface, cursor, v_scroll, h_scroll) = match &self.nodes[active_index].ty {
            NodeType::Buffer {
                buffer_id,
                surface,
                cursor,
                v_scroll,
                h_scroll,
            } => (*buffer_id, *surface, *cursor, *v_scroll, *h_scroll),

            NodeType::Split { .. } => {
                return Err(WorkspaceError::InvalidCaller);
            }
        };

        // promote active node to a split node
        let first_id = self.next_node_id;
        self.next_node_id += 1;
        let second_id = self.next_node_id;
        self.next_node_id += 1;

        self.nodes[active_index].ty = NodeType::Split {
            mode,
            first_id,
            second_id,
            area: surface,
        };

        // create children from the splitted surface
        let (first_surface, second_surface) = Self::split_rect(surface, mode);
        let first_child = Node {
            id: first_id,
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
            id: second_id,
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
        self.nodes.push(first_child);
        self.nodes.push(second_child);

        Ok(())
    }

    fn quit_active(&mut self) -> Result<(), WorkspaceError> {
        let active_id = match self.active_id {
            Some(id) => id,
            None => return Err(WorkspaceError::NullActive),
        };
        let active_index = self.get_node_index(active_id)?;

        // return error if calling from a split node
        if let NodeType::Split { .. } = &self.nodes[active_index].ty {
            return Err(WorkspaceError::InvalidCaller);
        };

        // remove active from nodes if root
        let parent_id = match self.nodes[active_index].parent_id {
            Some(id) => id,
            None => {
                self.nodes.swap_remove(active_index);
                self.active_id = None;
                return Ok(());
            }
        };
        let parent_index = self.get_node_index(parent_id)?;

        let (area, sibling_id) = match &self.nodes[parent_index].ty {
            NodeType::Split {
                area,
                first_id,
                second_id,
                ..
            } => {
                let id = if active_id == *first_id {
                    second_id
                } else if active_id == *second_id {
                    first_id
                } else {
                    return Err(WorkspaceError::InvalidTree(
                        "Node references an incoherent parent".to_string(),
                    ));
                };

                (*area, *id)
            }
            NodeType::Buffer { .. } => {
                return Err(WorkspaceError::InvalidTree(
                    "Buffer node has a buffer node parent".to_string(),
                ));
            }
        };
        let sibling_index = self.get_node_index(sibling_id)?;

        // update siblings children's parent_id if any
        match &self.nodes[sibling_index].ty {
            NodeType::Split {
                first_id,
                second_id,
                ..
            } => {
                let first_index = self.get_node_index(*first_id)?;
                let second_index = self.get_node_index(*second_id)?;

                self.nodes[first_index].parent_id = Some(parent_id);
                self.nodes[second_index].parent_id = Some(parent_id);
            }
            NodeType::Buffer { .. } => {}
        }
        // promote parent to whatever type sibling is
        self.nodes[parent_index].ty = self.nodes[sibling_index].ty.clone();
        self.restore_geometry_recursive(parent_id, area);

        // guard against any reordering of self.nodes
        self.nodes.swap_remove(self.get_node_index(active_id)?);
        self.nodes.swap_remove(self.get_node_index(sibling_id)?);

        Ok(())
    }

    fn restore_geometry_recursive(
        &mut self,
        id: NodeId,
        free_area: Rect,
    ) -> Result<(), WorkspaceError> {
        let index = self.get_node_index(id)?;

        // fecth split contents or directly consume the space if buffer
        let (mode, first_id, second_id) = match &mut self.nodes[index].ty {
            NodeType::Split {
                mode,
                first_id,
                second_id,
                area: _,
            } => (*mode, *first_id, *second_id),
            NodeType::Buffer { surface, .. } => {
                *surface = free_area;
                self.active_id = Some(id);
                return Ok(());
            }
        };

        let (first_area, second_area) = Self::split_rect(free_area, mode);

        // call recursively on children
        self.restore_geometry_recursive(first_id, first_area)?;
        self.restore_geometry_recursive(second_id, second_area)?;

        Ok(())
    }

    fn get_node_index(&self, id: NodeId) -> Result<usize, WorkspaceError> {
        self.nodes
            .iter()
            .position(|node| node.id == id)
            .ok_or(WorkspaceError::InvalidNodeId)
    }

    fn get_buff_index(&self, id: BufferId) -> Result<usize, WorkspaceError> {
        self.buffers
            .iter()
            .position(|node| node.id == id)
            .ok_or(WorkspaceError::InvalidBufferId)
    }

    fn split_rect(rect: Rect, mode: SplitMode) -> (Rect, Rect) {
        match mode {
            SplitMode::Vertical => {
                (
                    // left rect
                    Rect {
                        x: rect.x,
                        y: rect.y,
                        width: rect.width * 0.5,
                        height: rect.height,
                    },
                    // right rect (offset in x axis)
                    Rect {
                        x: rect.x + rect.width * 0.5,
                        y: rect.y,
                        width: rect.width * 0.5,
                        height: rect.height,
                    },
                )
            }
            SplitMode::Horizontal => {
                (
                    // top rect
                    Rect {
                        x: rect.x,
                        y: rect.y,
                        width: rect.width,
                        height: rect.height * 0.5,
                    },
                    // bottom rect (offset in y axis)
                    Rect {
                        x: rect.x,
                        y: rect.y + rect.height * 0.5,
                        width: rect.width,
                        height: rect.height * 0.5,
                    },
                )
            }
        }
    }
}
