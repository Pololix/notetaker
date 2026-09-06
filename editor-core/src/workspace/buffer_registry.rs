use crate::workspace::buffer::{Buffer, BufferId};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct BufferRegistry {
    buffers: HashMap<BufferId, Buffer>,
}

impl BufferRegistry {}
