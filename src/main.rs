#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
mod entity;
mod job_handler;
mod examples;
mod threadpool;
mod win;
mod graphics;
mod gamestate;

#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

use win::interface::*;
use graphics::graphics::*;

use job_handler::*;
use gamestate::*;

use vulkano_win::VkSurfaceBuild;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::device::Device;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::instance::Instance;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use std::sync::Arc;
use std::mem;

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
    handler.set_gamestate(gamestate);
    handler.set_threadpool(&10);

    let sender = handler.get_sender();

    loop {
        if Interface::poll_event_loop(&mut events_loop, &sender) {
            return;
        }
    }
}
