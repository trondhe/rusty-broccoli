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
