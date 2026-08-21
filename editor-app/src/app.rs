use editor_core::{
    Editor,
    event::input_event::{InputEvent, Key, KeyState, Modifiers},
};
use editor_renderer::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::NamedKey,
    window::{Window, WindowId},
};

type WinitKey = winit::keyboard::Key;

pub struct App {
    window_id: Option<WindowId>,
    renderer: Option<Renderer>,
    editor: Editor,

    current_mods: Modifiers,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);

        self.window_id = Some(window.id());
        let size = window.inner_size();
        let viewport = (size.width, size.height);

        self.renderer = match Renderer::new(window, viewport) {
            Ok(instance) => Some(instance),
            Err(error) => {
                println!("{error}");
                return;
            }
        };
        self.editor.set_viewport(viewport);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let input_event = match event {
            // exceptions (left out of the cmd/event system)
            WindowEvent::CloseRequested => {
                event_loop.exit();
                return;
            }
            WindowEvent::RedrawRequested => {
                let renderer = match &mut self.renderer {
                    Some(renderer) => renderer,
                    None => return,
                };
                let editor_view = self.editor.get_view();

                todo!();
            }
            WindowEvent::Resized(size) => {
                let renderer = match &mut self.renderer {
                    Some(renderer) => renderer,
                    None => return,
                };

                let viewport = (size.width, size.height);
                renderer.set_viewport(viewport);
                self.editor.set_viewport(viewport);
                return;
            }
            WindowEvent::ModifiersChanged(mods) => {
                let state = mods.state();

                self.current_mods.shift = state.shift_key();
                self.current_mods.ctrl = state.control_key();
                self.current_mods.alt = state.alt_key();
                self.current_mods.super_key = state.super_key();

                return;
            }
            // keyboard input
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => InputEvent::Key {
                key: match event.logical_key {
                    WinitKey::Character(str) => Key::Character(str.to_string()),
                    WinitKey::Named(key) => match key {
                        NamedKey::Space => Key::Space,
                        NamedKey::Enter => Key::Enter,
                        NamedKey::Escape => Key::Escape,
                        NamedKey::Backspace => Key::Backspace,
                        NamedKey::Tab => Key::Tab,
                        NamedKey::Delete => Key::Delete,

                        NamedKey::ArrowLeft => Key::Left,
                        NamedKey::ArrowRight => Key::Right,
                        NamedKey::ArrowUp => Key::Up,
                        NamedKey::ArrowDown => Key::Down,

                        NamedKey::F1 => Key::F(1),
                        NamedKey::F2 => Key::F(2),
                        NamedKey::F3 => Key::F(3),
                        NamedKey::F4 => Key::F(4),
                        NamedKey::F5 => Key::F(5),
                        NamedKey::F6 => Key::F(6),
                        NamedKey::F7 => Key::F(7),
                        NamedKey::F8 => Key::F(8),
                        NamedKey::F9 => Key::F(9),
                        NamedKey::F10 => Key::F(10),
                        NamedKey::F11 => Key::F(11),
                        NamedKey::F12 => Key::F(12),

                        _ => return, // for unused named keys
                    },
                    _ => return, // for unknown/dead keys
                },
                state: match event.state {
                    ElementState::Pressed => KeyState::Pressed,
                    ElementState::Released => KeyState::Released,
                },
                mods: self.current_mods,
            },
            _ => return,
        };
        // mouse/touch input

        self.editor.handle_input_event(input_event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // keep the loop running
        if let Some(id) = &self.window_id {
            self.window_event(&event_loop, *id, WindowEvent::RedrawRequested);
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            window_id: None,
            renderer: None,
            editor: Editor::new(),

            current_mods: Modifiers::default(),
        }
    }
}
