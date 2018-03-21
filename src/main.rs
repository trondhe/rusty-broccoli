#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
mod entity;
mod job_handler;
mod examples;
mod threadpool;
mod win;
mod graphics;

extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use win::interface;
use graphics::graphics::Graphics;

use std::sync::Arc;

fn main() {
    let mut events_loop = interface::make_events_loop();

    let instance = Graphics::get_instance();
    let physical = Graphics::get_physical(&instance);

    let window_config = interface::WindowConfig {
        title: String::from("Rusty Broccoli"),
        width: 400,
        height: 400,
        events_loop: &mut events_loop,
        instance: &instance,
    };
    let window = interface::make_window(&window_config);

    let queue = Graphics::get_queue(physical, &window);
    let (device, queues) = Graphics::get_device(physical, queue);

    interface::start_event_loop(window_config.events_loop);
}
