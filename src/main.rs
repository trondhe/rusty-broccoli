#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
mod entity;
mod job_handler;
mod examples;
mod threadpool;
mod win;

extern crate winit;

use win::interface;

fn main() {
    let mut events_loop = interface::make_events_loop();

    let window = interface::make_window(
        "Rusty Broccoli",
        400,
        400,
        &mut events_loop
    );

    interface::start_event_loop(&mut events_loop);
}
