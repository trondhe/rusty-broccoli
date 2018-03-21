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
    pub events_loop: &'a mut EventsLoop,
    pub instance: &'a Arc<Instance>
}

pub fn window_event_handler(window_event: WindowEvent) {
    match window_event {
        WindowEvent::Resized(w, h) => {
            println!("Window resized to {}x{}", w, h);
        }
        WindowEvent::KeyboardInput { device_id, input } => keyboard_input_handler(input),
        _ => (),
    }
}

pub fn keyboard_input_handler(input: KeyboardInput) {
    if let Some(keycode) = input.virtual_keycode {
        println!("{:?} was pressed", keycode);
    }
}

pub fn make_events_loop() -> EventsLoop {
    EventsLoop::new()
}

pub fn make_window(config: &WindowConfig) -> Arc<Surface<Window>> {
    let builder = WindowBuilder::new()
        .with_dimensions(config.width, config.height)
        .with_title(config.title.clone());

    builder.build_vk_surface(
        config.events_loop,
        config.instance.clone()
    ).unwrap()
}

pub fn start_event_loop(events_loop: &mut EventsLoop) {
    loop {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { window_id, event } => window_event_handler(event),
            _ => (),
        });
    }
}
