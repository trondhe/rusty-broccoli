//
// How to use:
//
// let instance = Graphics::get_instance();
// let physical = Graphics::get_physical(&instance);
//
// ... create a window instance
//
// let queue = Graphics::get_queue(physical, &window);
//
// ... display window/loop etc.
//

use vulkano_win;
use vulkano;

use vulkano::instance::{
    Instance,
    PhysicalDevice,
    QueueFamily,
};

use vulkano::device::{
    Device,
    QueuesIter,
};

use std::sync::Arc;

pub struct Graphics { }

impl Graphics {
    pub fn get_instance() -> Arc<Instance> {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None)
            .expect("Failed to create a Vulkan instance.")
    }

    pub fn get_physical(instance: &Arc<Instance>) -> PhysicalDevice {
        PhysicalDevice::enumerate(instance)
            .next()
            .expect("No physical device available.")
    }

    pub fn get_queue<'a>(physical: PhysicalDevice<'a>, window: &vulkano_win::Window) -> QueueFamily<'a> {
        physical
            .queue_families()
            .find(|&q| {
                let is_surface_supported = window.surface()
                    .is_supported(q)
                    .unwrap_or(false);

                q.supports_graphics() && is_surface_supported
            })
            .expect("Could not find a graphical queue family.")
    }

    pub fn get_device<'a>(physical: PhysicalDevice<'a>, queue: QueueFamily<'a>) -> (Arc<Device>, QueuesIter) {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };

        Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue, 0.5)].iter().cloned()
        ).expect("Failed to create device.")
    }
}
