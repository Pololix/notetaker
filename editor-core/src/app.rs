use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct App {
    id: Option<WindowId>,
    state: Option<editor_renderer::RendererState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);
        let (width, height) = (window.inner_size().width, window.inner_size().height);

        self.id = Some(window.id());
        self.state = Some(editor_renderer::RendererState::new(window, width, height));
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
                if let Some(state) = &mut self.state {
                    state.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &self.state {
                    let quads = vec![
                        editor_renderer::Quad {
                            x: 100,
                            y: 100,
                            width: 150,
                            height: 100,
                            color: editor_renderer::Color {
                                r: 1.0,
                                g: 0.2,
                                b: 0.2,
                                a: 1.0,
                            }, // red
                        },
                        editor_renderer::Quad {
                            x: 400,
                            y: 250,
                            width: 100,
                            height: 150,
                            color: editor_renderer::Color {
                                r: 0.2,
                                g: 1.0,
                                b: 0.2,
                                a: 1.0,
                            }, // green
                        },
                        editor_renderer::Quad {
                            x: 250,
                            y: 400,
                            width: 200,
                            height: 60,
                            color: editor_renderer::Color {
                                r: 0.2,
                                g: 0.4,
                                b: 1.0,
                                a: 1.0,
                            }, // blue
                        },
                    ];

                    state.render(&quads);
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
