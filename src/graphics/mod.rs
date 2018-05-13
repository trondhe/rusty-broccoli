// #[cfg(any(feature = "vulkan"))]
use backend;
use hal;
use hal::format::{ChannelType, Format, Swizzle};
use hal::pool;
use hal::{Backbuffer, Device, Instance, PhysicalDevice, Surface, SwapchainConfig};

use winit::Window;

pub struct Graphics {}

const COLOR_RANGE: hal::image::SubresourceRange = hal::image::SubresourceRange {
    aspects: hal::format::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

impl Graphics {
    pub fn setup(name: &str, window: &Window) {
        // TODO: Into environment variable, or something.
        let app_version = 1;

        let instance = backend::Instance::create(name, app_version);
        let mut surface = instance.create_surface(&window);
        let mut adapters = instance.enumerate_adapters();

        // TODO: Better handling of mulitple adapters
        let adapter = adapters.remove(0);
        let (surface_capabilties, format_vec) =
            surface.capabilities_and_formats(&adapter.physical_device);
        let surface_format = format_vec.map_or(Format::Rgba8Srgb, |formats| {
            formats
                .into_iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .unwrap()
        });

        let memory_types = adapter.physical_device.memory_properties().memory_types;

        let (mut device, mut queue_group) = adapter
            .open_with::<_, hal::General>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        let max_buffers = 4;
        let mut general_pool = device.create_command_pool_typed(
            &queue_group,
            pool::CommandPoolCreateFlags::empty(),
            max_buffers,
        );
        let mut queue = &mut queue_group.queues[0];

        let swap_config = SwapchainConfig::new()
            .with_color(surface_format)
            .with_image_usage(hal::image::Usage::COLOR_ATTACHMENT);

        let (mut swap_chain, backbuffer) = device.create_swapchain(&mut surface, swap_config);

        let frame_images = match backbuffer {
            Backbuffer::Images(images) => images
                .into_iter()
                .map(|image| {
                    let rtv = device
                        .create_image_view(
                            &image,
                            hal::image::ViewKind::D2,
                            surface_format,
                            Swizzle::NO,
                            COLOR_RANGE,
                        )
                        .unwrap();
                    (image, rtv)
                })
                .collect::<Vec<_>>(),
            _ => unimplemented!(),
        };

        let mut frame_semaphore = device.create_semaphore();
        let mut frame_fence = device.create_fence(false);
    }
}
