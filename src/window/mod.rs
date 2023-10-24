use vulkano::swapchain::SurfaceApi;

use crate::init::{VulkanConfig, VulkanInit};
use vulkano::{instance::Instance, VulkanLibrary};

use vulkano::device::{DeviceCreateInfo, QueueCreateInfo};

use vulkano::device::Device;

use std::sync::Arc;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use vulkano_win::VkSurfaceBuild;

use crate::init::VulkanInitError;
use vulkano::{
    instance::{InstanceCreateInfo, InstanceExtensions},
    swapchain::Surface,
};
#[cfg(all(unix, feature = "wayland"))]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Wayland;
}

#[cfg(all(unix, feature = "x11"))]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Xlib;
}

#[cfg(windows)]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Win32;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Metal;
}

#[cfg(target_os = "android")]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Android;
}

impl VulkanInit {
    #[cfg(feature = "winit")]
    pub fn new_winit<T>(
        vulkan_config: VulkanConfig,
        event_loop: &EventLoop<T>,
    ) -> Result<(Self, Arc<Surface>), VulkanInitError> {
        let library = VulkanLibrary::new()?;
        let instance_extensions = vulkano_win::required_extensions(&library);

        let mut ici = InstanceCreateInfo {
            ..vulkan_config.get_create_instance_info()
        };
        ici.enabled_extensions = InstanceExtensions {
            ..instance_extensions
        };

        let instance = Instance::new(library.clone(), ici)?;

        let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone())?;

        let (physical_device, queue_family_index) = Self::select_physical_device(
            instance.clone(),
            &None,
            vulkan_config.get_required_extensions(),
        )?;
        let (device, queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: vulkan_config.get_required_extensions(),
                ..Default::default()
            },
        )?;

        let vec_queues = Vec::from_iter(queues);

        Ok((
            Self {
                library: library.clone(),
                instance: instance.clone(),
                physical_device: physical_device.clone(),
                queues: vec_queues,
                device: device.clone(),
                config: vulkan_config,
            },
            surface.clone(),
        ))
    }
}
