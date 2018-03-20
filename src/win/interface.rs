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

pub fn start_event_loop() {
    let mut events_loop = EventsLoop::new();
    let builder = WindowBuilder::new()
        .with_dimensions(400, 400)
        .with_title("rusty-brocolli");
    let window = builder.build(&events_loop).unwrap();

    loop {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { window_id, event } => window_event_handler(event),
            _ => (),
        });
    }
}
