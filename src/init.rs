use std::sync::Arc;
use thiserror::Error;
use vulkano::{
    device::{
        physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceCreationError, Queue,
        QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo, InstanceCreationError, InstanceExtensions},
    LoadingError, VulkanError, VulkanLibrary,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct VulkanInit {
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    queues: Vec<Arc<Queue>>,
}

impl VulkanInit {
    pub fn new(instance_extensions: InstanceExtensions) -> Result<Self, VulkanInitError> {
        let library = VulkanLibrary::new()?;
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

#[derive(Error, Debug)]
pub enum VulkanInitError {
    #[error("failed to load the vulkano library")]
    LibraryLoadingError(#[from] LoadingError),
    #[error("could not find any suitable physical device")]
    NoPhysicalDeviceError,
    #[error("failed to create an vulkan instance")]
    InstanceInitError(#[from] InstanceCreationError),
    #[error("vulkan error")]
    VulkanError(#[from] VulkanError),
    #[error("could not find any suitable queues")]
    NoSuitableQueuesFound,
    #[error("could not create device")]
    DeviceCreationError(#[from] DeviceCreationError),
}
