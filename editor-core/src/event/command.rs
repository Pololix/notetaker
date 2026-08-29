use std::any::Any;

pub trait EditorCommand: Any {
    fn as_any(&self) -> &dyn Any;
}
