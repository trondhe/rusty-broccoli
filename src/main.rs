#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
mod entity;
mod examples;
mod gamestate;
mod graphics;
mod job_handler;
mod threadpool;
mod win;

#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

use graphics::graphics::*;
use win::interface::*;

use gamestate::*;
use job_handler::*;

use std::thread::sleep;
use std::thread::sleep_ms;
use std::time::Duration;

use vulkano_win::VkSurfaceBuild;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::device::Device;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::instance::Instance;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use std::mem;
use std::sync::Arc;

fn main() {
    let mut events_loop = Interface::make_events_loop();
    let instance = Graphics::get_instance();
    let physical = Graphics::get_physical(&instance);

    let window_config = WindowConfig {
        title: String::from("Wallaballa"),
        width: 800,
        height: 600,
        instance: &instance,
    };

    let surface = Interface::make_window(&window_config, &events_loop);

    // use std::thread;
    // use std::sync::mpsc;
    // use std::sync::Arc;
    // use std::sync::Mutex;
    // use std::sync::RwLock;
    // use std::time;

    // let thread = thread::spawn(move || {
    //     loop {

    //         // Fetch latest gamestate.

    //         // Draw gamestate.

    //         if let Ok(_) = receiver.try_recv() {
    //             println!("Exiting!");
    //             return;
    //         }
    //     }
    // });

    let gamestate = GameState::new();

    let mut handler = JobHandler::new();
    handler.set_gamestate(gamestate.clone());
    handler.set_threadpool(&10);

    let sender = handler.get_sender();

    let interface = Interface {
        gamestate: gamestate.clone(),
        sender: sender.clone(),
    };

    loop {
        let gs2 = gamestate.clone();
        let job = Box::new(move || {
            let state = gs2.read().unwrap();
            println!("{:?}", state.keyboard_state.key_map.get("key_a"));
        });
        sender.send(threadpool::Message::NewJob(job)).unwrap();
        sleep_ms(1);
        if interface.poll_event_loop(&mut events_loop) {
            return;
        }
    }
}
