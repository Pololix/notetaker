// TODO
// - make the API error-concerned
// - add padding between buffers and margins
// - make API config-driven
// - assert tree invariants

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

#[derive(Debug)]
enum NodeType {
    Split {
        mode: SplitMode,
        first_id: NodeId,
        second_id: NodeId,
    },
    Buffer {
        buffer_id: BufferId,
        rect: Rect,
    },
}

#[derive(Debug)]
pub struct Workspace {
    nodes: Vec<Node>,
    next_node_id: NodeId,
    buffers: Vec<Buffer>,
    next_buff_id: BufferId,
    active_id: Option<NodeId>, // if none should be empty
}

impl Workspace {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        // create default empty buffer
        let default_buffer = Buffer::new(0);
        // attach it to a node and focus
        let root_node = Node {
            id: 0,
            parent_id: None,
            ty: NodeType::Buffer {
                buffer_id: default_buffer.id,
                rect: Rect {
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
                    ty: NodeType::Buffer {
                        buffer_id: default_buffer.id,
                        rect: Rect {
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
        let (buffer_id, rect) = match &self.nodes[active_index].ty {
            NodeType::Buffer { buffer_id, rect } => (*buffer_id, *rect),
            NodeType::Split {
                mode: _,
                first_id: _,
                second_id: _,
            } => return, // split must be called from a buffer
        };

        // promote active to split mode
        let first_id = self.next_node_id;
        self.next_node_id += 1;
        let second_id = self.next_node_id;
        self.next_node_id += 1;

        self.nodes[active_index].ty = NodeType::Split {
            mode: split_mode,
            first_id,
            second_id,
        };

        // calculate new geometry
        // note: (0,0) at top left corner
        let (first_rect, second_rect) = match split_mode {
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
        };

        // create children
        // note: by default the reference the same buffer on creation
        let first_child = Node {
            id: first_id,
            parent_id: Some(active_id),
            ty: NodeType::Buffer {
                buffer_id,
                rect: first_rect,
            },
        };
        self.nodes.push(first_child);
        let second_child = Node {
            id: second_id,
            parent_id: Some(active_id),
            ty: NodeType::Buffer {
                buffer_id,
                rect: second_rect,
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

        // fetch id and geometry if buffer node
        let (buffer_id, rect) = match &self.nodes[active_index].ty {
            NodeType::Buffer { buffer_id, rect } => (*buffer_id, *rect),
            NodeType::Split {
                mode: _,
                first_id: _,
                second_id: _,
            } => return, // split must be called from a buffer
        };

        todo!("Finish delete function")
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
