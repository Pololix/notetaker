use egui::Context;
use egui_winit::State;
use ntk_core::Editor;
use ntk_renderer::Renderer;
use ntk_ui::Ui;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub struct AppState {
    window: Arc<Window>,
    window_id: WindowId,

    egui_ctx: Context,
    egui_winit: State,

    renderer: Renderer,
    editor: Editor,
}

#[derive(Default)]
pub struct Application(Option<AppState>);

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes();

        let window = Arc::new(
            event_loop
                .create_window(window_attrs)
                .expect("failed to create window"),
        );
        let window_id = window.id();

        let egui_ctx = Context::default();
        let egui_winit = State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        self.0 = Some(AppState {
            window,
            window_id,

            egui_ctx,
            egui_winit,

            renderer: Renderer::new(),
            editor: Editor::new(),
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // guards
        let Some(state) = &mut self.0 else {
            return;
        };
        if state.window_id != window_id {
            return;
        }

        // fetch response from egui layer
        let egui_response = state.egui_winit.on_window_event(&state.window, &event);
        if egui_response.repaint {
            state.window.request_redraw();
        }
        if egui_response.consumed {
            return;
        }

        // use if not already consumed
        match event {
            // rendering
            WindowEvent::RedrawRequested => Self::handle_redraw(state),
            WindowEvent::Resized(size) => Self::handle_resize(state, size.width, size.height),

            // input

            // closing
            WindowEvent::CloseRequested => event_loop.exit(),

            // unused
            _ => {}
        }
    }
}

impl Application {
    pub fn run() {
        let event_loop = EventLoop::new().expect("Failed to create an event loop");
        event_loop.set_control_flow(event_loop::ControlFlow::Poll);

        let mut app = Self::default();
        event_loop
            .run_app(&mut app)
            .expect("Failure during event loop execution");
    }

    fn handle_redraw(state: &AppState) {
        // gather input from last frame and build new one
        let input = state.egui_winit.take_egui_input(&state.window);
        let output = state.egui_ctx.run_ui(input, |ctx| ntk_ui::build());

        state
            .egui_winit
            .handle_platform_output(&state.window, output.platform_output);

        // prepare rendering material
        let primitives = state
            .egui_ctx
            .tessellate(output.shapes, output.pixels_per_point);
        let (width, height) = (
            state.window.inner_size().width,
            state.window.inner_size().height,
        );

        // update any textures if necessary and render
        state.renderer.update_egui_textures(&output.textures_delta);
        state
            .renderer
            .render(&primitives, width, height, output.pixels_per_point);
    }

    fn handle_resize(state: &AppState, width: u32, height: u32) {}

    fn handle_input() {}
}
