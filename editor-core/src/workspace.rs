// TODO
// - make the API error-concerned
// - add padding between buffers and margins
// - make API config-driven
// - assert tree invariants
// - modify to allow resizing buffer surfaces
// - rethink create/new buffer relation and introduce errors
// - movement between buffers (only view nodes)

use crate::{
    buffer::{Buffer, BufferId, BufferView},
    event::workspace_event::WorkspaceCommand,
};
use editor_common::{Rect, Viewport};

type NodeId = usize;
pub type WorkspaceId = usize;

#[derive(Debug, Clone, Copy)]
pub enum SplitMode {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    pub id: NodeId,
    parent_id: Option<NodeId>,
    pub ty: NodeType,
}

#[derive(Debug, Clone, Copy)]
enum NodeType {
    Split {
        mode: SplitMode,
        first_id: NodeId,
        second_id: NodeId,
        area: Rect,
    },
    Buffer {
        buffer_id: BufferId,
        surface: Rect,
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
        // create default empty buffer
        let default_buffer = Buffer::new(0);
        // attach it to a node and focus
        let root_node = Node {
            id: 0,
            parent_id: None,
            ty: NodeType::Buffer {
                buffer_id: default_buffer.id,
                surface: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: viewport.width as f32,
                    height: viewport.height as f32,
                },
            },
        };

        // store
        let buffers = vec![default_buffer];
        let nodes = vec![root_node];

        Self {
            id,
            nodes,
            next_node_id: 1,
            buffers,
            next_buff_id: 1,
            active_id: Some(0),
        }
    }

    pub fn adapt_to_viewport(&mut self, viewport: Viewport) {
        // fecth root and restore the geometry workspace-wide
        let root = self
            .nodes
            .iter()
            .find(|node| node.parent_id == None)
            .expect("Failed to retrieve root");
        self.restore_geometry_recursive(
            root.id,
            Rect {
                x: 0.0,
                y: 0.0,
                width: viewport.width as f32,
                height: viewport.height as f32,
            },
        );
    }

    pub fn get_view(&self) -> WorkspaceView {
        let buffer_views = self
            .nodes
            .iter()
            .filter_map(|node| match node.ty {
                NodeType::Buffer { buffer_id, surface } => {
                    let buffer_index = self.get_buff_index(buffer_id);
                    let buffer = &self.buffers[buffer_index];

                    Some(buffer.get_view(surface))
                }
                NodeType::Split { .. } => None,
            })
            .collect();

        WorkspaceView { buffer_views }
    }

    pub fn handle_command(&mut self, cmd: WorkspaceCommand) {
        match cmd {
            WorkspaceCommand::OpenBuffer { viewport } => self.add_buffer(viewport),
            WorkspaceCommand::CloseBuffer => self.delete_active(),
            WorkspaceCommand::SplitBuffer { mode } => self.split_active(mode),
            _ => return,
        }
    }

    fn add_buffer(&mut self, viewport: Viewport) {
        match self.active_id {
            Some(_) => {
                // by default the new split is vertical
                self.split_active(SplitMode::Vertical);
            }
            None => {
                // create default buffer
                let default_buffer = Buffer::new(self.next_buff_id);
                self.next_buff_id += 1;
                // attach it to a node and focus
                let new_root_node = Node {
                    id: self.next_node_id,
                    parent_id: None,
                    ty: NodeType::Buffer {
                        buffer_id: default_buffer.id,
                        surface: Rect {
                            x: 0.0,
                            y: 0.0,
                            width: viewport.width as f32,
                            height: viewport.height as f32,
                        },
                    },
                };

                self.next_node_id += 1;
                self.active_id = Some(new_root_node.id);
                self.buffers.push(default_buffer);
                self.nodes.push(new_root_node);
            }
        }
    }

    fn split_active(&mut self, split_mode: SplitMode) {
        let active_id = match self.active_id {
            Some(id) => id,
            None => return, // no active node to split from
        };
        let active_index = self.get_node_index(active_id);

        // fetch id and geometry if buffer node
        let (buffer_id, surface) = match &self.nodes[active_index].ty {
            NodeType::Buffer { buffer_id, surface } => (*buffer_id, *surface),
            NodeType::Split { .. } => return, // split must be called from a buffer
        };

        // promote active to split mode
        let first_id = self.next_node_id;
        self.next_node_id += 1;
        let second_id = self.next_node_id;
        self.next_node_id += 1;

        // now the buffer rect is used to be splitted into children
        self.nodes[active_index].ty = NodeType::Split {
            mode: split_mode,
            first_id,
            second_id,
            area: surface,
        };

        // calculate new geometry
        // note: (0,0) at top left corner
        let (first_surface, second_surface) = match split_mode {
            SplitMode::Vertical => {
                (
                    // left rect
                    Rect {
                        x: surface.x,
                        y: surface.y,
                        width: surface.width * 0.5,
                        height: surface.height,
                    },
                    // right rect (offset in x axis)
                    Rect {
                        x: surface.x + surface.width * 0.5,
                        y: surface.y,
                        width: surface.width * 0.5,
                        height: surface.height,
                    },
                )
            }
            SplitMode::Horizontal => {
                (
                    // top rect
                    Rect {
                        x: surface.x,
                        y: surface.y,
                        width: surface.width,
                        height: surface.height * 0.5,
                    },
                    // bottom rect (offset in y axis)
                    Rect {
                        x: surface.x,
                        y: surface.y + surface.height * 0.5,
                        width: surface.width,
                        height: surface.height * 0.5,
                    },
                )
            }
        };

        // create children
        // note: by default the reference the same buffer on creation
        let first_child = Node {
            id: first_id,
            parent_id: Some(active_id),
            ty: NodeType::Buffer {
                buffer_id,
                surface: first_surface,
            },
        };
        self.nodes.push(first_child);

        let second_child = Node {
            id: second_id,
            parent_id: Some(active_id),
            ty: NodeType::Buffer {
                buffer_id,
                surface: second_surface,
            },
        };
        self.nodes.push(second_child);

        // note: by default new pops at right/bottom
        self.active_id = Some(second_id);
    }

    fn delete_active(&mut self) {
        let active_id = match self.active_id {
            Some(id) => id,
            None => return, // no active node to delete
        };
        let active_index = self.get_node_index(active_id);

        // fetch id if buffer node
        let buffer_id = match &self.nodes[active_index].ty {
            NodeType::Buffer {
                buffer_id,
                surface: _,
            } => *buffer_id,
            NodeType::Split { .. } => return, // split must be called from a buffer
        };
        let buffer_index = self.get_buff_index(buffer_id);

        // delete whole if root
        let parent_id = match self.nodes[active_index].parent_id {
            Some(id) => id,
            None => {
                self.nodes.swap_remove(active_index);
                self.buffers.swap_remove(buffer_index); // no other should be viewing it
                return;
            }
        };
        let parent_index = self.get_node_index(parent_id);

        // fetch sibling id from parent (split node)
        let (sibling_id, parent_area) = match &self.nodes[parent_index].ty {
            NodeType::Split {
                mode: _,
                first_id,
                second_id,
                area,
            } => {
                let id = if active_id == *first_id {
                    *second_id
                } else if active_id == *second_id {
                    *first_id
                } else {
                    return; // buffer is pointing to another parent
                };

                (id, *area)
            }
            NodeType::Buffer { .. } => return, // parent of the buffer is a buffer
        };
        let sibling_index = self.get_node_index(sibling_id);

        // promote parent to sibling type (with data), preserving the parent node
        self.nodes[parent_index].ty = self.nodes[sibling_index].ty.clone();

        // update siblings children if any
        match &self.nodes[parent_index].ty {
            NodeType::Split {
                mode: _,
                first_id,
                second_id,
                area: _,
            } => {
                let first_index = self.get_node_index(*first_id);
                let second_index = self.get_node_index(*second_id);
                self.nodes[first_index].parent_id = Some(parent_id);
                self.nodes[second_index].parent_id = Some(parent_id);
            }
            NodeType::Buffer { .. } => {}
        }

        // guard against any reordering of self.nodes
        self.nodes.swap_remove(self.get_node_index(active_id));
        self.nodes.swap_remove(self.get_node_index(sibling_id));

        self.restore_geometry_recursive(parent_id, parent_area);
    }

    fn restore_geometry_recursive(&mut self, id: NodeId, free_area: Rect) {
        let index = self.get_node_index(id);

        // fecth split contents or directly consume the space if buffer
        let (mode, first_id, second_id) = match &mut self.nodes[index].ty {
            NodeType::Split {
                mode,
                first_id,
                second_id,
                area: _,
            } => (*mode, *first_id, *second_id),
            NodeType::Buffer {
                buffer_id: _,
                surface,
            } => {
                *surface = free_area;
                self.active_id = Some(id);
                return;
            }
        };

        let (first_area, second_area) = match mode {
            SplitMode::Vertical => {
                (
                    // left rect
                    Rect {
                        x: free_area.x,
                        y: free_area.y,
                        width: free_area.width * 0.5,
                        height: free_area.height,
                    },
                    // right rect (offset in x axis)
                    Rect {
                        x: free_area.x + free_area.width * 0.5,
                        y: free_area.y,
                        width: free_area.width * 0.5,
                        height: free_area.height,
                    },
                )
            }
            SplitMode::Horizontal => {
                (
                    // top rect
                    Rect {
                        x: free_area.x,
                        y: free_area.y,
                        width: free_area.width,
                        height: free_area.height * 0.5,
                    },
                    // bottom rect (offset in y axis)
                    Rect {
                        x: free_area.x,
                        y: free_area.y + free_area.height * 0.5,
                        width: free_area.width,
                        height: free_area.height * 0.5,
                    },
                )
            }
        };

        // call recursively on children
        self.restore_geometry_recursive(first_id, first_area);
        self.restore_geometry_recursive(second_id, second_area);
    }

    fn get_node_index(&self, id: NodeId) -> usize {
        self.nodes
            .iter()
            .position(|node| node.id == id)
            .expect("Failed to retrieve node index from id")
    }

    fn get_buff_index(&self, id: BufferId) -> usize {
        self.buffers
            .iter()
            .position(|node| node.id == id)
            .expect("Failed to retrieve node index from id")
    }
}
