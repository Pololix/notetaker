use editor_core::Editor;
use editor_renderer::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App {
    window_id: Option<WindowId>,
    _renderer: Option<Renderer>,
    _editor: Editor,
}

impl ApplicationHandler for App {
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

impl App {
    pub fn new() -> Self {
        Self {
            window_id: None,
            _renderer: None,
            _editor: Editor::new().expect("Failed to create a new editor instance"),
        }
    }
}
