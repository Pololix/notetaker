use ntk_core::{
    Editor,
    platform::PlatformEvent,
    render::{RenderProtocol, Viewport},
};
use ntk_renderer::Renderer;
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct AppState {
    window: Arc<Window>,

    frame_time: Instant,
    frame_events: Vec<PlatformEvent>,

    renderer: Renderer,
    editor: Editor,
}

#[derive(Default)]
pub struct Application(Option<AppState>);

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.0.is_some() {
            return;
        }

        let window_attrs = Window::default_attributes();
        let window = Arc::new(
            event_loop
                .create_window(window_attrs)
                .expect("failed to create window"),
        );

        let size = window.inner_size();
        let viewport = Viewport::new(size.width, size.height);

        let renderer = Renderer::new(window.clone(), viewport).unwrap();
        let editor = Editor::new(viewport);

        self.0 = Some(AppState {
            window,

            frame_time: Instant::now(),
            frame_events: Vec::new(),

            renderer,
            editor,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.0 else {
            return;
        };
        if state.window.id() != window_id {
            return;
        }

        match event {
            // rendering
            WindowEvent::RedrawRequested => state.frame_events.push(PlatformEvent::RedrawRequested),
            WindowEvent::Resized(size) => {
                state
                    .frame_events
                    .push(PlatformEvent::WindowResized(Viewport::new(
                        size.width,
                        size.height,
                    )))
            }

            // input

            // closing
            WindowEvent::CloseRequested => {
                for event in state.frame_events.drain(..) {
                    state.editor.handle_platform_event(event);
                }

                state
                    .editor
                    .handle_platform_event(PlatformEvent::ExitRequested);
                event_loop.exit();
            }

            // unused
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(state) = &mut self.0 else {
            return;
        };

        // fecth frame timestamp
        let now = Instant::now();
        let dt = (now - state.frame_time).as_secs_f32();
        state.frame_time = now;

        // handle platform events
        for event in state.frame_events.drain(..) {
            state.editor.handle_platform_event(event);
        }

        // update core and render its state
        state.editor.update(dt);
        state.renderer.render(&state.editor.render());
    }
}

impl Application {
    pub fn run() {
        let event_loop = EventLoop::new().expect("Failed to create an event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = Self::default();
        event_loop
            .run_app(&mut app)
            .expect("Failure during event loop execution");
    }
}
