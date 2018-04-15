use vulkano_win;
use vulkano;

use winit::Window;

use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};

use vulkano::device::{Device, Queue, QueuesIter};

use vulkano::swapchain::{Capabilities, PresentMode, Surface, SurfaceTransform, Swapchain};

use vulkano::sync::SharingMode;

use vulkano::image::SwapchainImage;

use vulkano_win::VkSurfaceBuild;

use std::sync::Arc;

pub struct SwapchainConfig<'a> {
    pub surface: &'a Arc<Surface<Window>>,
    pub capabilities: &'a Capabilities,
    pub device: &'a Arc<Device>,
    pub queue: &'a Arc<Queue>,
}

pub struct Graphics {}

impl Graphics {
    pub fn get_instance() -> Arc<Instance> {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("Failed to create a Vulkan instance.")
    }

    pub fn get_physical(instance: &Arc<Instance>) -> PhysicalDevice {
        PhysicalDevice::enumerate(instance)
            .next()
            .expect("No physical device available.")
    }

    pub fn get_queue<'a>(
        physical: PhysicalDevice<'a>,
        surface: &Surface<Window>,
    ) -> QueueFamily<'a> {
        physical
            .queue_families()
            .find(|&q| {
                let is_surface_supported = surface.is_supported(q).unwrap_or(false);

                q.supports_graphics() && is_surface_supported
            })
            .expect("Could not find a graphical queue family.")
    }

    pub fn get_device<'a>(
        physical: PhysicalDevice<'a>,
        queue: QueueFamily<'a>,
    ) -> (Arc<Device>, QueuesIter) {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::none()
        };

        Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue, 0.5)].iter().cloned(),
        ).expect("Failed to create device.")
    }

    pub fn get_capabilities<'a>(
        surface: &Arc<Surface<Window>>,
        physical: PhysicalDevice<'a>,
    ) -> Capabilities {
        surface
            .capabilities(physical)
            .expect("Failed to get surface capabilities.")
    }

    pub fn get_dimensions(capabilities: &Capabilities) -> [u32; 2] {
        capabilities.current_extent.unwrap_or([1024, 768])
    }

    pub fn get_swapchain(
        config: &SwapchainConfig, dimensions: [u32; 2]
    ) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
        let alpha = config
            .capabilities
            .supported_composite_alpha
            .iter()
            .next()
            .unwrap();

        let format = config.capabilities.supported_formats[0].0;

        Swapchain::new(
            config.device.clone(),
            config.surface.clone(),
            config.capabilities.min_image_count,
            format,
            dimensions,
            1,
            config.capabilities.supported_usage_flags,
            SharingMode::from(config.queue),
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            true,
            None,
        ).expect("Failed to create swapchain.")
    }
}

pub fn gfxloop() {
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

    // loop {
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
    // }
}
