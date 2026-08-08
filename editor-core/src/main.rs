use winit::event_loop::EventLoop;

mod app;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Error during event loop execution");
}
