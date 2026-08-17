use crate::splits::buffer::Buffer;

// change the whole API to take errors into account and not panic at the minimal inconvenience
// all configs will be done with u32
// all gpu (clip-space) and cpu (fractional pixel space) callcuolations will use f32
// in pixels
const MARGIN: u32 = 5; //from edges to elements
const PADDING: u32 = 5; //between elements

#[derive(Debug, Clone, Copy)]
enum BufferSplitMode {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
enum BufferTreeNodeType {
    // link references both its parent (link) and both children (buffers)
    Split(usize, usize, BufferSplitMode),
    // buffer references its parent (link) af any (is root)
    Buffer(Buffer),
}

#[derive(Debug)]
struct BufferTreeNode {
    pub id: usize,
    pub parent_id: Option<usize>,
    pub ty: BufferTreeNodeType,
}

#[derive(Debug)]
pub struct Workspace {
    //id: usize,
    next_node_id: usize,
    nodes: Vec<BufferTreeNode>,
    active_node: Option<usize>,
}

impl Workspace {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        // create root node with buffer automatically
        let root = BufferTreeNode {
            id: 0,
            parent_id: None,
            ty: BufferTreeNodeType::Buffer(Buffer::new(
                PADDING as f32,
                PADDING as f32,
                (window_width - 2 * PADDING) as f32,
                (window_height - 2 * PADDING) as f32,
            )),
        };

        Self {
            next_node_id: 1,
            nodes: vec![root],
            active_node: Some(0),
        }
    }

    // change to act upon option active node (create or create at split)
    pub fn new_buffer(&mut self, window_width: u32, window_height: u32) {
        // must only trigger if no open buffers
        // root should be non-existent
        let new_root = BufferTreeNode {
            id: self.next_node_id,
            parent_id: None,
            ty: BufferTreeNodeType::Buffer(Buffer::new(
                PADDING as f32,
                PADDING as f32,
                (window_width - 2 * PADDING) as f32,
                (window_height - 2 * PADDING) as f32,
            )),
        };
        self.next_node_id += 1;
        self.active_node = Some(new_root.id);
        self.nodes.push(new_root);
    }

    pub fn split_active_buffer(&mut self, mode: BufferSplitMode) {
        // early return for if no active node from which to split
        if let None = self.active_node {
            return;
        }

        let active_node_id = self.active_node.unwrap();
        let active_node_index = self.get_index(active_node_id);
        if let BufferTreeNodeType::Buffer(buffer) = &self.nodes[active_node_index].ty {
            let buffer = buffer.clone();

            // calculate children positions and size
            let child_size: (f32, f32);
            let new_origin: (f32, f32);
            match mode {
                BufferSplitMode::Vertical => {
                    child_size = ((buffer.width - PADDING as f32) * 0.5, buffer.height);
                    new_origin = ((buffer.x + child_size.0 + PADDING as f32), buffer.y);
                }
                BufferSplitMode::Horizontal => {
                    child_size = (buffer.width, (buffer.height - PADDING as f32) * 0.5);
                    new_origin = (buffer.x, (buffer.y + child_size.1 + PADDING as f32));
                }
            }

            // promote active node to split node
            let child_keep_id = self.next_node_id;
            self.next_node_id += 1;
            let child_new_id = self.next_node_id;
            self.next_node_id += 1;

            self.nodes[active_node_index].ty =
                BufferTreeNodeType::Split(child_keep_id, child_new_id, mode);

            // create children nodes
            // child_keep keeps original (resized) buffer
            let mut buffer = buffer;
            buffer.width = child_size.0;
            buffer.height = child_size.1;
            let child_keep = BufferTreeNode {
                id: child_keep_id,
                parent_id: Some(active_node_id),
                ty: BufferTreeNodeType::Buffer(buffer),
            };
            self.nodes.push(child_keep);
            // child_new gets handed a new one
            let child_new = BufferTreeNode {
                id: child_new_id,
                parent_id: Some(active_node_id),
                ty: BufferTreeNodeType::Buffer(Buffer::new(
                    new_origin.0,
                    new_origin.1,
                    child_size.0,
                    child_size.1,
                )),
            };
            self.nodes.push(child_new);
        }
    }

    pub fn write_active_buffer(&mut self) {}

    pub fn quit_active_buffer(&mut self) {
        // // early return for if no active node from which to split
        // let active_id = match self.active_node {
        //     Some(id) => id,
        //     None => return,
        // };
        // let active_index = self.get_index(active_id);
        //
        // // early return if (somehow) the caller is not a buffer
        // // fecth parent_id and quit buffer
        // let (freed_geometry, parent_id) = match &mut self.nodes[active_index].ty {
        //     BufferTreeNodeType::Buffer(buffer) => {
        //         let freed_geometry = buffer.quit();
        //         let parent_id = self.nodes[active_index].parent_id;
        //         self.nodes.remove(active_index);
        //
        //         (freed_geometry, parent_id)
        //     }
        //     BufferTreeNodeType::Split(_, _, _) => return,
        // };
        //
        // // if root wipe state
        // let parent_id = match parent_id {
        //     Some(id) => id,
        //     None => {
        //         self.nodes.clear();
        //         self.active_node = None;
        //         return;
        //     }
        // };
        // let parent_index = self.get_index(parent_id);
        //
        // // if not promote sibling to parent
        // let sibling_id = match &self.nodes[parent_index].ty {
        //     BufferTreeNodeType::Split(child1_id, child2_id, _) => {
        //         let id = if active_id == *child1_id {
        //             *child2_id
        //         } else if active_id == *child2_id {
        //             *child1_id
        //         } else {
        //             return; // active isnt a child of its parent
        //         };
        //
        //         id
        //     }
        //     BufferTreeNodeType::Buffer(_) => return, //parent cannot be buffer
        // };
        // let sibling_index = self.get_index(sibling_id);
        // self.nodes[parent_index].ty = self.nodes[sibling_index].ty.clone();
        // self.nodes.remove(sibling_index);
        //
        // // restore geometry
        // self.restore_geometry_recursive(parent_id, freed_geometry);
    }

    fn get_index(&self, id: usize) -> usize {
        self.nodes
            .iter()
            .position(|node| node.id == id)
            .expect("Failed to retrieve node index from id")
    }

    fn restore_geometry_recursive(&mut self, id: usize, free_geometry: (f32, f32, f32, f32)) {
        let index = self.get_index(id);
        match self.nodes[index].ty.clone() {
            // if is a split, split the free space according to split mode
            BufferTreeNodeType::Split(child1_id, child2_id, split_mode) => match split_mode {
                BufferSplitMode::Vertical => {}
                BufferSplitMode::Horizontal => {}
            },

            // if is a buffer take up the rest of the space
            BufferTreeNodeType::Buffer(mut buffer) => {
                // the space is left/right
                if free_geometry.2 < buffer.width {
                    buffer.x = buffer.x.min(free_geometry.0);
                    buffer.width += free_geometry.2 + PADDING as f32;
                // the space is up down
                } else {
                    buffer.y = buffer.y.min(free_geometry.1);
                    buffer.height += free_geometry.3 + PADDING as f32;
                }
            }
        }
    }
}
