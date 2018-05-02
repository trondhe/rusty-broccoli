// #[cfg(any(feature = "vulkan"))]
use backend;
use hal;
use hal::format::{ChannelType, Format};
use hal::pool;
use hal::{Device, Instance, PhysicalDevice, Surface};

use winit::Window;

pub struct Graphics {}

impl Graphics {
    pub fn setup(name: &str, window: &Window) {
        // TODO: Into environment variable, or something.
        let app_version = 1;

        let instance = backend::Instance::create(name, app_version);
        let surface = instance.create_surface(&window);
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
    }
}
