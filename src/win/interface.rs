use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowBuilder, WindowEvent};

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

pub fn make_window(title: &str, width: u32, height: u32, events_loop: &mut EventsLoop) -> Window {
    let builder = WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title(title);

    builder.build(events_loop).unwrap()
}

pub fn start_event_loop(events_loop: &mut EventsLoop) {
    loop {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { window_id, event } => window_event_handler(event),
            _ => (),
        });
    }
}
