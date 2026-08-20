use editor_core::{Editor, EditorInputEvent};
use editor_renderer::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

#[derive(Debug)]
pub struct App {
    window_id: Option<WindowId>,
    renderer: Option<Renderer>,
    editor: Editor,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WinitWindow::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);

        self.window_id = Some(window.id());
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.translate_event(event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(id) = &self.window_id {
            self.window_event(&event_loop, *id, WindowEvent::RedrawRequested);
        }
    }
}

impl Window {
    pub fn new() -> Self {
        Self {
            window_id: None,
            renderer: None,
            editor: Editor::new(),
        }
    }

    fn translate_window_event(&self, event: WindowEvent) {
        let input_event = match event {
            _ => {
                println!("No functionality added for event: {}", event);
                return;
            }
        };

        todo!("populare window event matches");
        // self.editor.handle_input_event(input_event);
    }
}
