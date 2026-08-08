use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use editor_renderer::RendererState;

#[derive(Default)]
pub struct App {
    state: Option<RendererState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);
        let (width, height) = (window.inner_size().width, window.inner_size().height);

        self.state = Some(RendererState::new(window, width, height));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => println!("redraw logic"),
            _ => {}
        }
    }
}
