use vulkano::swapchain::SurfaceApi;

use crate::init::{VulkanConfig, VulkanInit};
use vulkano::{instance::Instance, VulkanLibrary};

use vulkano::{
    device::{DeviceCreateInfo, QueueCreateInfo},
    instance::InstanceCreateInfo,
};

use vulkano::device::{Device, QueueFlags};

use crate::init::NoPhysicalDeviceError;
use crate::init::VulkanInitError;
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
    pub fn new_winit(vulkan_config: VulkanConfig) -> Result<Self, VulkanInitError> {
        let library = VulkanLibrary::new()?;
        let instance_extensions = vulkano_win::required_extensions(&library);
        let mut instance_create_info = vulkan_config.get_create_instance_info();

        instance_create_info.enabled_extensions = instance_create_info
            .enabled_extensions
            .intersection(&instance_extensions);

        let instance = Instance::new(library.clone(), instance_create_info)?;
        let physical_device = instance
            .enumerate_physical_devices()?
            .filter(|p| {
                p.supported_extensions()
                    .contains(&vulkan_config.get_required_extensions())
            })
            .next()
            .ok_or_else(|| NoPhysicalDeviceError("no physical vulkan device found".to_owned()))?;

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .ok_or(VulkanInitError::NoSuitableQueuesFound)? as u32;
        let (device, queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )?;

        let vec_queues = Vec::from_iter(queues);

        Ok(Self {
            library: library.clone(),
            instance: instance.clone(),
            physical_device: physical_device.clone(),
            queues: vec_queues,
            device: device.clone(),
            config: vulkan_config,
        })
    }
}
