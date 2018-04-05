#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
mod entity;
mod job_handler;
mod examples;
mod threadpool;
mod win;
mod graphics;

#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

use win::interface::*;
use graphics::graphics::*;

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
    // let instance = Graphics::get_instance();
    // let physical = Graphics::get_physical(&instance);

    // let window_config = WindowConfig {
    //     title: String::from("Wallaballa"),
    //     width: 800,
    //     height: 600,
    //     instance: &instance,
    // };

    // let surface = Interface::make_window(&window_config, &events_loop);
    // let queue = Graphics::get_queue(physical, &surface);
    // let (device, mut queues) = Graphics::get_device(physical, queue);

    // let queue = queues.next().unwrap();

    // let capabilities = Graphics::get_capabilities(&surface, physical);
    // let mut dimensions = Graphics::get_dimensions(&capabilities);

    // let swapchain_config = SwapchainConfig {
    //     surface: &surface,
    //     capabilities: &capabilities,
    //     device: &device,
    //     queue: &queue,
    // };

    // let (mut swapchain, mut images) =
    //     Graphics::get_swapchain(&swapchain_config, dimensions.clone());

    // let mut x_pos: f32 = 0.0;

    // mod vs {
    //     #[derive(VulkanoShader)]
    //     #[ty = "vertex"]
    //     #[src = "
    //         #version 450

    //         layout(location = 0) in vec2 position;

    //         void main() {
    //             gl_Position = vec4(position, 0.0, 1.0);
    //         }
    //     "]
    //     struct Dummy;
    // }

    // mod fs {
    //     #[derive(VulkanoShader)]
    //     #[ty = "fragment"]
    //     #[src = "
    //         #version 450

    //         layout(location = 0) out vec4 f_color;

    //         void main() {
    //             f_color = vec4(1.0, 0.0, 0.0, 1.0);
    //         }
    //     "]
    //     struct Dummy;
    // }

    // let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    // let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    // let render_pass = Arc::new(
    //     single_pass_renderpass!(device.clone(),
    //     attachments: {
    //         color: {
    //             load: Clear,
    //             store: Store,
    //             format: swapchain.format(),
    //             samples: 1,
    //         }
    //     },
    //     pass: {
    //         color: [color],
    //         depth_stencil: {}
    //     }
    // ).unwrap(),
    // );

    // let pipeline = Arc::new(
    //     GraphicsPipeline::start()
    //         .vertex_input_single_buffer()
    //         .vertex_shader(vs.main_entry_point(), ())
    //         .triangle_list()
    //         .viewports_dynamic_scissors_irrelevant(1)
    //         .fragment_shader(fs.main_entry_point(), ())
    //         .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
    //         .build(device.clone())
    //         .unwrap(),
    // );

    // let mut framebuffers: Option<Vec<Arc<vulkano::framebuffer::Framebuffer<_, _>>>> = None;
    // let mut recreate_swapchain = false;
    // let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;

    use std::thread;
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::RwLock;
    use std::time;

    let (sender, receiver) = mpsc::channel();
    //let receiver = RwLock::new(receiver);
    // let thread = thread::spawn(move || {
    //     events_loop;
    //     thread::sleep(time::Duration::from_secs(10));
    //     sender.send(true).unwrap();
    // });
    unsafe impl Send for EventsLoop 
    let thread = thread::spawn(move || {
        //sender.send(true).unwrap();
        loop {
            events_loop.poll_events(|event| match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Closed,
                    ..
                } => sender.send(true).unwrap(),
                _ => (),
            });
        }
    });

    loop {
        // if Interface::poll_event_loop(&mut events_loop, &mut x_pos) {
        //     return;
        // }

        if let Ok(_) = receiver.try_recv() {
            println!("Exiting!");
            return;
        }

        // previous_frame_end.cleanup_finished();
        // if recreate_swapchain {
        //     dimensions = surface
        //         .capabilities(physical)
        //         .expect("failed to get surface capabilities")
        //         .current_extent
        //         .unwrap();

        //     let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
        //         Ok(r) => r,
        //         Err(SwapchainCreationError::UnsupportedDimensions) => {
        //             continue;
        //         }
        //         Err(err) => panic!("{:?}", err),
        //     };

        //     mem::replace(&mut swapchain, new_swapchain);
        //     mem::replace(&mut images, new_images);

        //     framebuffers = None;

        //     recreate_swapchain = false;
        // }
        // if framebuffers.is_none() {
        //     let new_framebuffers = Some(
        //         images
        //             .iter()
        //             .map(|image| {
        //                 Arc::new(
        //                     Framebuffer::start(render_pass.clone())
        //                         .add(image.clone())
        //                         .unwrap()
        //                         .build()
        //                         .unwrap(),
        //                 )
        //             })
        //             .collect::<Vec<_>>(),
        //     );
        //     mem::replace(&mut framebuffers, new_framebuffers);
        // }

        // let (image_num, acquire_future) =
        //     match swapchain::acquire_next_image(swapchain.clone(), None) {
        //         Ok(r) => r,
        //         Err(AcquireError::OutOfDate) => {
        //             recreate_swapchain = true;
        //             continue;
        //         }
        //         Err(err) => panic!("{:?}", err),
        //     };

        // let vertex_buffer = {
        //     #[derive(Debug, Clone)]
        //     struct Vertex {
        //         position: [f32; 2],
        //     }
        //     impl_vertex!(Vertex, position);

        //     CpuAccessibleBuffer::from_iter(
        //         device.clone(),
        //         BufferUsage::all(),
        //         [
        //             Vertex {
        //                 position: [-0.1 + x_pos, 0.5],
        //             },
        //             Vertex {
        //                 position: [-0.5 + x_pos, -0.5],
        //             },
        //             Vertex {
        //                 position: [0.5 + x_pos, -0.5],
        //             },
        //         ].iter()
        //             .cloned(),
        //     ).expect("failed to create buffer")
        // };

        // let command_buffer =
        //     AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
        //         .unwrap()
        //         .begin_render_pass(
        //             framebuffers.as_ref().unwrap()[image_num].clone(),
        //             false,
        //             vec![[0.0, 0.0, 1.0, 1.0].into()],
        //         )
        //         .unwrap()
        //         .draw(
        //             pipeline.clone(),
        //             DynamicState {
        //                 line_width: None,
        //                 viewports: Some(vec![
        //                     Viewport {
        //                         origin: [0.0, 0.0],
        //                         dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        //                         depth_range: 0.0..1.0,
        //                     },
        //                 ]),
        //                 scissors: None,
        //             },
        //             vertex_buffer.clone(),
        //             (),
        //             (),
        //         )
        //         .unwrap()
        //         .end_render_pass()
        //         .unwrap()
        //         .build()
        //         .unwrap();

        // let future = previous_frame_end
        //     .join(acquire_future)
        //     .then_execute(queue.clone(), command_buffer)
        //     .unwrap()
        //     .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
        //     .then_signal_fence_and_flush();

        // match future {
        //     Ok(future) => {
        //         previous_frame_end = Box::new(future) as Box<_>;
        //     }
        //     Err(vulkano::sync::FlushError::OutOfDate) => {
        //         recreate_swapchain = true;
        //         previous_frame_end = Box::new(vulkano::sync::now(device.clone())) as Box<_>;
        //     }
        //     Err(e) => {
        //         println!("{:?}", e);
        //         previous_frame_end = Box::new(vulkano::sync::now(device.clone())) as Box<_>;
        //     }
        // }
    }
}
