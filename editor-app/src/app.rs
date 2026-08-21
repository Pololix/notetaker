use editor_core::{Editor, EditorInputEvent};
use editor_renderer::{Renderer, RendererError};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App {
    window_id: Option<WindowId>,
    renderer: Option<Renderer>,
    editor: Editor,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);

        self.window_id = Some(window.id());
        let size = window.inner_size();
        let viewport = (size.width, size.height);

        self.renderer = match Renderer::new(window, viewport) {
            Ok(instance) => Some(instance),
            Err(error) => {
                println!("{error}");
                return;
            }
        };
        self.editor.set_viewport(viewport);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let input_event = match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                let renderer = match &mut self.renderer {
                    Some(renderer) => renderer,
                    None => return,
                };

                let viewport = (size.width, size.height);
                renderer.set_viewport(viewport);
                self.editor.set_viewport(viewport);
                return;
            }
            WindowEvent::RedrawRequested => {
                let renderer = match &mut self.renderer {
                    Some(renderer) => renderer,
                    None => return,
                };
                let quads = self.editor.render_active();
                renderer.render(&quads);
                return;
            }
            _ => return,
        };

        // self.editor.handle_input_event(input_event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // keep the loop running
        if let Some(id) = &self.window_id {
            self.window_event(&event_loop, *id, WindowEvent::RedrawRequested);
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            window_id: None,
            renderer: None,
            editor: Editor::new(),
        }
    }
}
