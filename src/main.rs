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
use graphics::graphics::*;

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
    let surface = interface::make_window(&window_config);

    let queue = Graphics::get_queue(physical, &surface);
    let (device, mut queues) = Graphics::get_device(physical, queue);

    let queue = queues.next().unwrap();

    let capabilities = Graphics::get_capabilities(&surface, physical);
    let dimensions = Graphics::get_dimensions(&capabilities);

    let swapchain_config = SwapchainConfig {
        surface: &surface,
        capabilities: &capabilities,
        dimensions: &dimensions,
        device: &device,
        queue: &queue,
    };
    let (mut swapchain, mut images) = Graphics::get_swapchain(&swapchain_config);

    interface::start_event_loop(window_config.events_loop);
}
