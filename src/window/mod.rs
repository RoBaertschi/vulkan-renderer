use vulkano::swapchain::SurfaceApi;

use crate::init::VulkanInit;
use vulkano::{instance::Instance, VulkanLibrary};

use vulkano::{
    device::{DeviceCreateInfo, QueueCreateInfo},
    instance::InstanceCreateInfo,
};

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
    pub fn new_winit() -> Result<Self, VulkanInitError> {
        let library = VulkanLibrary::new()?;
        let instance_extensions = vulkano_win::required_extensions(&library);
        let instance = Instance::new(
            library.clone(),
            InstanceCreateInfo {
                enabled_extensions: instance_extensions,
                ..Default::default()
            },
        )?;
        let physical_device = instance
            .enumerate_physical_devices()?
            .next()
            .ok_or_else(|| VulkanInitError::NoPhysicalDeviceError)?;

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
        })
    }
}
