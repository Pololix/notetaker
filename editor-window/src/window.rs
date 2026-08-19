use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

type WinitWindow = winit::window::Window;

pub struct Window {
    id: Option<WindowId>,
}

impl Default for Window {
    fn default() -> Self {
        Self { id: None }
    }
}

impl ApplicationHandler for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WinitWindow::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            _ => println!("No functionality added for {:?}", event),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(id) = &self.id {
            self.window_event(&event_loop, *id, WindowEvent::RedrawRequested);
        }
    }
}

impl Window {
    pub fn run(&mut self) /* -> Result<(), Err> */
    {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop
            .run_app(self)
            .expect("Error during event loop execution");
    }
}
