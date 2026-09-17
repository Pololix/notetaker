use egui::{Context, ViewportId};
use egui_winit::State;
use ntk_common::Viewport;
use ntk_core::{Editor, Viewport};
use ntk_renderer::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct AppState {
    window_id: WindowId,
    window: Arc<Window>,
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
            WindowEvent::RedrawRequested => Self::handle_redraw(state),
            WindowEvent::Resized(size) => Self::handle_resize(state, size),

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
        if let Some(state) = &self.0 {
            state.window.request_redraw();
        }
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

    fn handle_redraw(state: &AppState) {
        let frame = state.editor.render();
        state.renderer.render(frame);
    }

    fn handle_resize(state: &AppState, size: PhysicalSize<u32>) {
        let viewport = Viewport::new(size.width, size.height);
        todo!("Handle resize");
    }

    fn handle_input(state: &AppState, input_event: WindowEvent) {
        match event {
            _ => {}
        }
    }
}
