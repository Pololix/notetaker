use editor_core::editor::Editor;
use editor_renderer::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App<'a> {
    window_id: Option<WindowId>,
    _renderer: Option<Renderer<'a>>,
    _editor: Editor,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);

        self.window_id = Some(window.id());
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            window_id: None,
            _renderer: None,
            _editor: Editor::new().expect("Failed to create an editor"),
        }
    }
}
