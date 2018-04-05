use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowBuilder, WindowEvent};

use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use vulkano_win;
use vulkano_win::VkSurfaceBuild;

use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Sender;

use threadpool;
use gamestate::GameState;

pub struct WindowConfig<'a> {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub instance: &'a Arc<Instance>,
}

pub struct Interface<'a> {
    gamestate: Arc<RwLock<GameState<'a>>>,
}

impl<'a> Interface<'a> {
    pub fn poll_event_loop(
        &self,
        events_loop: &mut EventsLoop,
        sender: &Arc<Sender<threadpool::Message>>,
    ) -> bool {
        let mut done: bool = false;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::Closed,
                ..
            } => done = true,
            Event::WindowEvent { window_id, event } => {
                Interface::window_event_handler(self, event, sender)
            }
            _ => (),
        });
        done
    }

    pub fn window_event_handler(
        &self,
        window_event: WindowEvent,
        sender: &Arc<Sender<threadpool::Message>>,
    ) {
        match window_event {
            WindowEvent::Resized(w, h) => {
                println!("Window resized to {}x{}", w, h);
            }
            WindowEvent::KeyboardInput { device_id, input } => {
                Interface::keyboard_input_handler(self, input, sender)
            }
            _ => (),
        }
    }

    pub fn keyboard_input_handler(
        &self,
        input: KeyboardInput,
        sender: &Arc<Sender<threadpool::Message>>,
    ) {
        if let Some(keycode) = input.virtual_keycode {
            // match keycode {
            //     VirtualKeyCode::A => (),
            //     VirtualKeyCode::D => (),
            //     _ => (),
            // }

            if keycode == VirtualKeyCode::A {
                let job = Box::new(move || {
                    let mut state = self.gamestate.write().unwrap();
                    state.test_var += 1;
                    // println!("{:?} was pressed", keycode);
                });
                sender.send(threadpool::Message::NewJob(job)).unwrap();
            }
        }
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
