use editor_core::{
    Editor, EditorEvent,
    input_event::{InputEvent, Key, KeyPress, Mods},
    util::Viewport,
};
use editor_renderer::Renderer;
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

type WinitKey = winit::keyboard::Key;
type WinitNamedKey = winit::keyboard::NamedKey;

pub struct App {
    window_id: Option<WindowId>,
    _renderer: Option<Renderer>,
    editor: Editor,

    mods: Mods,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create a window");
        let window = Arc::new(window);

        self.window_id = Some(window.id());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.handle_window_event(event, event_loop);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.editor.update(Instant::now());
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            window_id: None,
            _renderer: None,
            editor: Editor::new().expect("Failed to create a new editor instance"),

            mods: Mods::empty(),
        }
    }

    fn handle_window_event(&mut self, event: WindowEvent, event_loop: &ActiveEventLoop) {
        let event = match event {
            // rendering
            WindowEvent::Resized(size) => {
                let viewport = Viewport {
                    width: size.width,
                    height: size.height,
                };
                EditorEvent::Resized(viewport)
            }
            WindowEvent::RedrawRequested => EditorEvent::RedrawRequested,

            // input
            WindowEvent::ModifiersChanged(mods) => {
                let state = mods.state();

                if state.shift_key() {
                    self.mods.with(Mods::SHIFT);
                } else {
                    self.mods.without(Mods::SHIFT);
                }

                if state.control_key() {
                    self.mods.with(Mods::CTRL);
                } else {
                    self.mods.without(Mods::CTRL);
                }

                if state.alt_key() {
                    self.mods.with(Mods::ALT);
                } else {
                    self.mods.without(Mods::ALT);
                }

                if state.super_key() {
                    self.mods.with(Mods::SUPER);
                } else {
                    self.mods.without(Mods::SUPER);
                }

                return;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if !event.state.is_pressed() {
                    return; // discard release events
                }

                let key = match event.logical_key {
                    WinitKey::Character(str) => Key::Character(str.to_string()),
                    WinitKey::Named(key) => match key {
                        WinitNamedKey::Space => Key::Space,
                        WinitNamedKey::Enter => Key::Enter,
                        WinitNamedKey::Escape => Key::Escape,
                        WinitNamedKey::Backspace => Key::Backspace,
                        WinitNamedKey::Tab => Key::Tab,
                        WinitNamedKey::Delete => Key::Delete,

                        WinitNamedKey::ArrowLeft => Key::Left,
                        WinitNamedKey::ArrowRight => Key::Right,
                        WinitNamedKey::ArrowUp => Key::Up,
                        WinitNamedKey::ArrowDown => Key::Down,

                        WinitNamedKey::F1 => Key::F(1),
                        WinitNamedKey::F2 => Key::F(2),
                        WinitNamedKey::F3 => Key::F(3),
                        WinitNamedKey::F4 => Key::F(4),
                        WinitNamedKey::F5 => Key::F(5),
                        WinitNamedKey::F6 => Key::F(6),
                        WinitNamedKey::F7 => Key::F(7),
                        WinitNamedKey::F8 => Key::F(8),
                        WinitNamedKey::F9 => Key::F(9),
                        WinitNamedKey::F10 => Key::F(10),
                        WinitNamedKey::F11 => Key::F(11),
                        WinitNamedKey::F12 => Key::F(12),

                        _ => return, // for unused named keys
                    },
                    _ => return, // for unknown/dead keys
                };

                EditorEvent::Input(InputEvent::Key(KeyPress {
                    key,
                    mods: self.mods,
                }))
            }
            WindowEvent::MouseInput { state, button, .. } => return,
            WindowEvent::MouseWheel { delta, .. } => return,
            WindowEvent::CursorMoved { position, .. } => return,
            // closing
            WindowEvent::CloseRequested => {
                event_loop.exit();
                return;
            }
            _ => return, // unused WindowEvents
        };

        self.editor.event_bus.push_event(event);
    }
}
