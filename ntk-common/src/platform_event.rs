use crate::Viewport;

#[derive(Debug, Clone)]
pub enum PlatformEvent {
    WindowResized(Viewport),
    RedrawRequested,
    ExitRequested,
}
