use crate::splits::Workspace;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App {
    id: Option<WindowId>,
    renderer: Option<editor_renderer::RendererState>,
    workspace: Workspace, // in a future a variable amount
}

impl Default for App {
    fn default() -> Self {
        Self {
            id: None,
            renderer: None,
            workspace: Workspace::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);
        let (width, height) = (window.inner_size().width, window.inner_size().height);

        self.id = Some(window.id());
        self.renderer = Some(editor_renderer::RendererState::new(window, width, height));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(state) = &mut self.renderer {
                    state.resize(size.width, size.height);
                }
            }
            _ => println!("No functionality added for {:?}", event),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(id) = &self.id {
            self.window_event(&event_loop, *id, WindowEvent::RedrawRequested);
        }
    }
}
