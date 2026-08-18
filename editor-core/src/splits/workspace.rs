// TODO
// - make the API error-concerned
// - add padding between buffers and margins
// - make API config-driven
// - assert tree invariants
// - modify to allow resizing buffer surfaces

use crate::splits::{
    buffer::{Buffer, BufferId},
    temp::Rect,
    utils::SplitMode,
};

type NodeId = usize;

#[derive(Debug)]
struct Node {
    id: NodeId,
    parent_id: Option<NodeId>,
    ty: NodeType,
}

#[derive(Debug, Clone)]
enum NodeType {
    Split {
        mode: SplitMode,
        first_id: NodeId,
        second_id: NodeId,
        area: Rect,
    },
    View {
        buffer_id: BufferId,
        surface: Rect,
    },
}

#[derive(Debug)]
pub struct Workspace {
    active_id: Option<NodeId>,
    nodes: Vec<Node>,
    next_node_id: NodeId,

    buffers: Vec<Buffer>,
    next_buff_id: BufferId,
}

impl Workspace {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        // create default empty buffer
        let default_buffer = Buffer::new(0);
        // attach it to a node and focus
        let root_node = Node {
            id: 0,
            parent_id: None,
            ty: NodeType::View {
                buffer_id: default_buffer.id,
                surface: Rect {
                    x: 0.0,
                    y: 0.0,
                    width: window_width as f32,
                    height: window_height as f32,
                },
            },
        };

        // store
        let buffers = vec![default_buffer];
        let nodes = vec![root_node];

        Self {
            nodes,
            next_node_id: 1,
            buffers,
            next_buff_id: 1,
            active_id: Some(0),
        }
    }

    pub fn add_buffer(&mut self, window_width: u32, window_height: u32) {
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
                    ty: NodeType::View {
                        buffer_id: default_buffer.id,
                        surface: Rect {
                            x: 0.0,
                            y: 0.0,
                            width: window_width as f32,
                            height: window_height as f32,
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

    pub fn split_active(&mut self, split_mode: SplitMode) {
        let active_id = match self.active_id {
            Some(id) => id,
            None => return, // no active node to split from
        };
        let active_index = self.get_node_index(active_id);

        // fetch id and geometry if buffer node
        let (buffer_id, surface) = match &self.nodes[active_index].ty {
            NodeType::View { buffer_id, surface } => (*buffer_id, *surface),
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
            ty: NodeType::View {
                buffer_id,
                surface: first_surface,
            },
        };
        self.nodes.push(first_child);
        let second_child = Node {
            id: second_id,
            parent_id: Some(active_id),
            ty: NodeType::View {
                buffer_id,
                surface: second_surface,
            },
        };
        // note: by default new pops at right/bottom
        self.active_id = Some(second_id);
        self.nodes.push(second_child);
    }

    pub fn delete_active(&mut self) {
        let active_id = match self.active_id {
            Some(id) => id,
            None => return, // no active node to delete
        };
        let active_index = self.get_node_index(active_id);

        // fetch id if buffer node
        let buffer_id = match &self.nodes[active_index].ty {
            NodeType::View {
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
            NodeType::View { .. } => return, // parent of the buffer is a buffer
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
            NodeType::View { .. } => {}
        }

        // restore the geometry and clean stale nodes
        self.restore_geometry_recursive(parent_id, parent_area);

        self.nodes.swap_remove(active_index);
        self.nodes.swap_remove(sibling_index);
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
            NodeType::View {
                buffer_id: _,
                surface,
            } => {
                *surface = free_area;
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
}
