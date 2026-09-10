use crate::event::input::{key::KeyPress, mouse::MousePress};

#[derive(Debug, Clone)]
pub enum InputEvent {
    Text(String),
    KeyPress(KeyPress),
    MousePress(MousePress),
    Scroll { delta_x: f32, delta_y: f32 },
}
