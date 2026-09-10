use crate::{
    document::{Camera, Document},
    user_mode::UserMode,
};

#[derive(Debug, Default)]
pub struct Editor {
    mode: UserMode,

    camera: Camera,
    document: Document,
}
