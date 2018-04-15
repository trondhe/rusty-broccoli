use winit::{ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowBuilder,
            WindowEvent};

use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use vulkano_win;
use vulkano_win::VkSurfaceBuild;

use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::RwLock;

use gamestate::GameState;
use gamestate::KeyState;
use threadpool;

pub struct WindowConfig<'a> {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub instance: &'a Arc<Instance>,
}

pub struct Interface {
    pub gamestate: Arc<RwLock<GameState>>,
    pub sender: Arc<Sender<threadpool::Message>>,
}

impl Interface {
    pub fn poll_event_loop(&self, events_loop: &mut EventsLoop) -> bool {
        let mut done: bool = false;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::Closed,
                ..
            } => done = true,
            Event::WindowEvent { window_id, event } => Interface::window_event_handler(self, event),
            _ => (),
        });
        done
    }

    pub fn window_event_handler(&self, window_event: WindowEvent) {
        match window_event {
            WindowEvent::Resized(w, h) => {
                println!("Window resized to {}x{}", w, h);
            }
            WindowEvent::KeyboardInput { device_id, input } => {
                Interface::keyboard_input_handler(self, input)
            }
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
                VirtualKeyCode::D => (),
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

    pub fn make_events_loop() -> EventsLoop {
        EventsLoop::new()
    }

    pub fn make_window(config: &WindowConfig, events_loop: &EventsLoop) -> Arc<Surface<Window>> {
        let builder = WindowBuilder::new()
            .with_dimensions(config.width, config.height)
            .with_title(config.title.clone());

        builder
            .build_vk_surface(events_loop, config.instance.clone())
            .unwrap()
    }
}
