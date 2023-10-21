use std::{default, sync::Arc};
use thiserror::Error;
use vulkano::{
    device::{
        physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceCreationError, DeviceExtensions,
        Queue, QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo, InstanceCreationError, InstanceExtensions},
    LoadingError, VulkanError, VulkanLibrary,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct VulkanInit {
    pub library: Arc<VulkanLibrary>,
    pub instance: Arc<Instance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queues: Vec<Arc<Queue>>,
    pub config: VulkanConfig,
}

impl VulkanInit {
    pub fn new(vulkan_config: VulkanConfig) -> Result<Self, VulkanInitError> {
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(library.clone(), vulkan_config.get_create_instance_info())?;
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

#[derive(Error, Debug)]
pub enum VulkanInitError {
    #[error("failed to load the vulkano library")]
    LibraryLoadingError(#[from] LoadingError),
    #[error("could not find any suitable physical device")]
    NoPhysicalDeviceError(#[from] NoPhysicalDeviceError),
    #[error("failed to create an vulkan instance")]
    InstanceInitError(#[from] InstanceCreationError),
    #[error("vulkan error")]
    VulkanError(#[from] VulkanError),
    #[error("could not find any suitable queues")]
    NoSuitableQueuesFound,
    #[error("could not create device")]
    DeviceCreationError(#[from] DeviceCreationError),
}

#[derive(Error, Debug)]
#[error("{}", .0)]
pub struct NoPhysicalDeviceError(pub String);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VulkanConfig {
    swap_chain: bool,
}

impl VulkanConfig {
    pub fn get_create_instance_info(&self) -> InstanceCreateInfo {
        InstanceCreateInfo {
            enabled_extensions: InstanceExtensions {
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn get_required_extensions(&self) -> DeviceExtensions {
        DeviceExtensions {
            khr_swapchain: self.swap_chain,
            ..DeviceExtensions::empty()
        }
    }
}
