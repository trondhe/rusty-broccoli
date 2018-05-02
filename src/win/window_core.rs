use winit::{ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowBuilder,
            WindowEvent};

use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::RwLock;

use gamestate::GameState;
use gamestate::KeyState;
use threadpool;

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl WindowConfig {
    pub fn new(title: &str, width: u32, height: u32) -> WindowConfig {
        WindowConfig {
            title: String::from(title),
            width: width,
            height: height,
        }
    }
}

pub struct WindowCore {
    gamestate: Arc<RwLock<GameState>>,
    sender: Arc<Sender<threadpool::Message>>,
    events_loop: EventsLoop,
    window: Window,
}

impl WindowCore {
    pub fn new(
        gamestate: Arc<RwLock<GameState>>,
        sender: Arc<Sender<threadpool::Message>>,
        window_config: WindowConfig,
    ) -> WindowCore {
        let events_loop = EventsLoop::new();

        let window_builder = WindowBuilder::new()
            .with_dimensions(window_config.width, window_config.height)
            .with_title(window_config.title);

        let window = window_builder.build(&events_loop).unwrap();

        WindowCore {
            gamestate: gamestate,
            sender: sender,
            events_loop,
            window,
        }
    }

    pub fn poll_event_loop(&mut self) -> bool {
        let mut exit_main_loop: bool = false;

        let mut event: Option<Event> = None;
        self.events_loop.poll_events(|e| {
            event = Some(e);
        });

        if let Some(event) = event {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::Closed,
                    ..
                } => exit_main_loop = true,
                Event::WindowEvent { window_id, event } => self.window_event_handler(event),
                _ => (),
            }
        }

        exit_main_loop
    }

    pub fn window_event_handler(&self, window_event: WindowEvent) {
        match window_event {
            WindowEvent::Resized(w, h) => {
                println!("Window resized to {}x{}", w, h);
            }
            WindowEvent::KeyboardInput { device_id, input } => self.keyboard_input_handler(input),
            _ => (),
        }
    }

    pub fn keyboard_input_handler(&self, input: KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            let key_state = match input.state {
                ElementState::Pressed => KeyState::Pressed,
                ElementState::Released => KeyState::Released,
            };
            match keycode {
                VirtualKeyCode::A => self.set_keystate("key_a", key_state),
                VirtualKeyCode::B => self.set_keystate("key_b", key_state),
                VirtualKeyCode::C => self.set_keystate("key_c", key_state),
                VirtualKeyCode::D => self.set_keystate("key_d", key_state),
                VirtualKeyCode::E => self.set_keystate("key_e", key_state),
                VirtualKeyCode::F => self.set_keystate("key_f", key_state),
                VirtualKeyCode::G => self.set_keystate("key_g", key_state),
                VirtualKeyCode::H => self.set_keystate("key_h", key_state),
                VirtualKeyCode::I => self.set_keystate("key_i", key_state),
                VirtualKeyCode::J => self.set_keystate("key_j", key_state),
                VirtualKeyCode::K => self.set_keystate("key_k", key_state),
                VirtualKeyCode::L => self.set_keystate("key_l", key_state),
                VirtualKeyCode::M => self.set_keystate("key_m", key_state),
                VirtualKeyCode::N => self.set_keystate("key_n", key_state),
                VirtualKeyCode::O => self.set_keystate("key_o", key_state),
                VirtualKeyCode::P => self.set_keystate("key_p", key_state),
                VirtualKeyCode::Q => self.set_keystate("key_q", key_state),
                VirtualKeyCode::R => self.set_keystate("key_r", key_state),
                VirtualKeyCode::S => self.set_keystate("key_s", key_state),
                VirtualKeyCode::T => self.set_keystate("key_t", key_state),
                VirtualKeyCode::U => self.set_keystate("key_u", key_state),
                VirtualKeyCode::V => self.set_keystate("key_v", key_state),
                VirtualKeyCode::W => self.set_keystate("key_w", key_state),
                VirtualKeyCode::X => self.set_keystate("key_x", key_state),
                VirtualKeyCode::Y => self.set_keystate("key_y", key_state),
                VirtualKeyCode::Z => self.set_keystate("key_z", key_state),
                VirtualKeyCode::Key1 => self.set_keystate("key_1", key_state),
                VirtualKeyCode::Key2 => self.set_keystate("key_2", key_state),
                VirtualKeyCode::Key3 => self.set_keystate("key_3", key_state),
                VirtualKeyCode::Key4 => self.set_keystate("key_4", key_state),
                VirtualKeyCode::Key5 => self.set_keystate("key_5", key_state),
                VirtualKeyCode::Key6 => self.set_keystate("key_6", key_state),
                VirtualKeyCode::Key7 => self.set_keystate("key_7", key_state),
                VirtualKeyCode::Key8 => self.set_keystate("key_8", key_state),
                VirtualKeyCode::Key9 => self.set_keystate("key_9", key_state),
                VirtualKeyCode::Key0 => self.set_keystate("key_0", key_state),
                VirtualKeyCode::LControl => self.set_keystate("key_ctrl", key_state),
                VirtualKeyCode::LShift => self.set_keystate("key_shift", key_state),
                VirtualKeyCode::LAlt => self.set_keystate("key_alt", key_state),
                VirtualKeyCode::Grave => self.set_keystate("key_pipe", key_state),
                VirtualKeyCode::Tab => self.set_keystate("key_tab", key_state),
                VirtualKeyCode::Capital => self.set_keystate("key_caps", key_state),
                VirtualKeyCode::Up => self.set_keystate("key_up", key_state),
                VirtualKeyCode::Down => self.set_keystate("key_down", key_state),
                VirtualKeyCode::Left => self.set_keystate("key_left", key_state),
                VirtualKeyCode::Right => self.set_keystate("key_right", key_state),
                VirtualKeyCode::Home => self.set_keystate("key_home", key_state),
                VirtualKeyCode::End => self.set_keystate("key_end", key_state),
                VirtualKeyCode::PageUp => self.set_keystate("key_pgup", key_state),
                VirtualKeyCode::PageDown => self.set_keystate("key_pgdown", key_state),
                VirtualKeyCode::Insert => self.set_keystate("key_insert", key_state),
                VirtualKeyCode::Delete => self.set_keystate("key_delete", key_state),
                VirtualKeyCode::Escape => self.set_keystate("key_esc", key_state),
                VirtualKeyCode::Back => self.set_keystate("key_backspace", key_state),
                VirtualKeyCode::Return => self.set_keystate("key_return", key_state),
                VirtualKeyCode::Space => self.set_keystate("key_space", key_state),
                _ => (),
            }
        }
    }

    pub fn set_keystate(&self, key: &'static str, state: KeyState) {
        let state_ref = self.gamestate.clone();
        let job = Box::new(move || {
            let mut game_state = state_ref.write().unwrap();
            game_state.update_keyboard_state(key, state);
        });
        self.sender.send(threadpool::Message::NewJob(job)).unwrap();
    }
}
