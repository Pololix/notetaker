use std::any::Any;

pub trait EditorEvent: Any {
    fn as_any(&self) -> &dyn Any;
}
