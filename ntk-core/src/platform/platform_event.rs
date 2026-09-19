use crate::render::Viewport;

#[derive(Debug, Clone)]
pub enum PlatformEvent {
    WindowResized(Viewport),
    RedrawRequested,
    ExitRequested,
}
