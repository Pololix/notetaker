mod app;
mod splits;
mod user_state;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::default();
    event_loop
        .run_app(&mut app)
        .expect("Error during event loop execution");
}
