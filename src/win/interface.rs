use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowBuilder, WindowEvent};

use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use vulkano_win;
use vulkano_win::VkSurfaceBuild;

use std::sync::Arc;

pub struct WindowConfig<'a> {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub instance: &'a Arc<Instance>,
}

pub struct Interface {}

impl Interface {
    pub fn poll_event_loop(events_loop: &mut EventsLoop, x_pos: &mut f32) -> bool {
        let mut done: bool = false;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::Closed,
                ..
            } => done = true,
            Event::WindowEvent { window_id, event } => {
                Interface::window_event_handler(event, x_pos)
            }
            _ => (),
        });
        done
    }

    pub fn window_event_handler(window_event: WindowEvent, x_pos: &mut f32) {
        match window_event {
            WindowEvent::Resized(w, h) => {
                println!("Window resized to {}x{}", w, h);
            }
            WindowEvent::KeyboardInput { device_id, input } => {
                Interface::keyboard_input_handler(input, x_pos)
            }
            _ => (),
        }
    }

    pub fn keyboard_input_handler(input: KeyboardInput, x_pos: &mut f32) {
        if let Some(keycode) = input.virtual_keycode {
            match keycode {
                VirtualKeyCode::A => *x_pos += 1.0,
                VirtualKeyCode::D => *x_pos -= 1.0,
                _ => (),
            }
            println!("{:?} was pressed", keycode);
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
