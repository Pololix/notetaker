use crate::splits::workspace::Workspace;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{self, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

pub struct App {
    id: Option<WindowId>,
    renderer: Option<editor_renderer::RendererState>,
    user: UserState,
    // change to hold a variable amount (1-9)
    workspace: Workspace,
}

impl Default for App {
    fn default() -> Self {
        Self {
            id: None,
            renderer: None,
            user: UserState::Normal,
            workspace: Workspace::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);
        let (width, height) = (window.inner_size().width, window.inner_size().height);

        self.id = Some(window.id());
        self.renderer = Some(editor_renderer::RendererState::new(window, width, height));
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
                if let Some(state) = &mut self.renderer {
                    state.resize(size.width, size.height);
                }
            }
            // WindowEvent::RedrawRequested => {
            //     if let Some(state) = &mut self.renderer {
            //         let quads: Vec<Quad> = vec![];
            //         state.render(&quads);
            //     }
            // }
            // WindowEvent::KeyboardInput {
            //     device_id: _,
            //     event,
            //     is_synthetic: _,
            // } => {
            //     // temp discard of release events
            //     if event.state == event::ElementState::Released {
            //         return;
            //     }
            //     match self.user {
            //         UserState::Normal => match event.logical_key {
            //             Key::Named(NamedKey::Escape) => {
            //                 self.user = UserState::Insert;
            //                 println!("Now in insert mode");
            //             }
            //             _ => {}
            //         },
            //         UserState::Insert => match event.logical_key {
            //             Key::Named(NamedKey::Escape) => {
            //                 self.user = UserState::Normal;
            //                 println!("Now in normal mode");
            //             }
            //             Key::Named(NamedKey::Backspace) => self.buffer.backspace(),
            //             Key::Named(NamedKey::Delete) => self.buffer.delete(),
            //             Key::Character(char) => {
            //                 for c in char.chars() {
            //                     self.buffer.insert(c);
            //                 }
            //             }
            //             Key::Named(NamedKey::ArrowLeft) => {
            //                 self.buffer.move_cursor(CursorDirection::Left)
            //             }
            //             Key::Named(NamedKey::ArrowRight) => {
            //                 self.buffer.move_cursor(CursorDirection::Right)
            //             }
            //             _ => {}
            //         },
            //         UserState::Cmdline => match event.logical_key {
            //             Key::Named(NamedKey::Escape) => {
            //                 self.user = UserState::Normal;
            //                 println!("Now in normal mode");
            //             }
            //             _ => {}
            //         },
            //     }
            // }
            _ => println!("No functionality added for {:?}", event),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(id) = &self.id {
            self.window_event(&event_loop, *id, WindowEvent::RedrawRequested);
        }
    }
}
