use ntk_common::{AppCommand, AppEvent};
use ntk_core::{Editor, EventBus, render::Viewport};
use ntk_renderer::Renderer;
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct AppState {
    window_id: WindowId,
    window: Arc<Window>,

    prev_frame: Instant,

    event_bus: EventBus<AppEvent, AppCommand>,
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
            window_id: window.id(),
            window,

            prev_frame: Instant::now(),

            event_bus: EventBus::default(),
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
        if state.window_id != window_id {
            return;
        }

        match event {
            // rendering
            WindowEvent::RedrawRequested => {}
            WindowEvent::Resized(size) => {}

            // input

            // closing
            WindowEvent::CloseRequested => {
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

        let now = Instant::now();
        let dt = (now - state.prev_frame).as_secs_f32();
        state.prev_frame = now;

        Self::update(state, dt);
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

    pub fn update(state: &mut AppState, dt: f32) {
        // commands produced by events are immediately processed
        let events = state.event_bus.get_events();
        for event in events {
            let mut command_writer = state.event_bus.get_command_writer();

            // process
        }

        // events produced by commands are stored for the next iteration
        let cmds = state.event_bus.get_commands();
        for cmd in cmds {
            let mut event_writer = state.event_bus.get_event_writer();

            // process
        }
    }
}
