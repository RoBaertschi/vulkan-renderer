use std::{sync::Arc, default};
use thiserror::Error;
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceCreationError, DeviceExtensions, Queue, QueueCreateInfo,
        QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo, InstanceCreationError, InstanceExtensions},
    swapchain::Surface,
    LoadingError, VulkanError, VulkanLibrary,
};
use vulkano_win::CreationError;

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

        Ok(Self {
            library: library.clone(),
            instance: instance.clone(),
            physical_device: physical_device.clone(),
            queues: vec_queues,
            device: device.clone(),
            config: vulkan_config,
        })
    }

    pub(super) fn select_physical_device(
        instance: Arc<Instance>,
        surface: &Option<Arc<Surface>>,
        device_extensions: DeviceExtensions,
    ) -> Result<(Arc<PhysicalDevice>, u32), VulkanInitError> {
        instance
            .enumerate_physical_devices()?
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    // Find the first first queue family that is suitable.
                    // If none is found, `None` is returned to `filter_map`,
                    // which disqualifies this physical device.
                    .position(|(i, q)| {
                        if surface.is_some() {
                            q.queue_flags.contains(QueueFlags::GRAPHICS)
                                && p.surface_support(i as u32, &surface.clone().unwrap())
                                    .unwrap_or(false)
                        } else {
                            q.queue_flags.contains(QueueFlags::GRAPHICS)
                        }
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,

                // Note that there exists `PhysicalDeviceType::Other`, however,
                // `PhysicalDeviceType` is a non-exhaustive enum. Thus, one should
                // match wildcard `_` to catch all unknown device types.
                _ => 4,
            })
            .ok_or_else(|| {
                NoPhysicalDeviceError("no physical vulkan device found".to_owned()).into()
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
    #[cfg(feature = "winit")]
    #[error("could not create window surface")]
    WinitCreationError(#[from] CreationError)
}

#[derive(Error, Debug)]
#[error("{}", .0)]
pub struct NoPhysicalDeviceError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct VulkanConfig {
    pub swap_chain: bool,
    pub support_moltenvk: bool,
}

impl Default for VulkanConfig {
    fn default() -> Self {
        Self {
            swap_chain: Default::default(),
            support_moltenvk: true
        }
    }
}

impl VulkanConfig {
    pub fn get_create_instance_info(&self) -> InstanceCreateInfo {
        InstanceCreateInfo {
            enabled_extensions: InstanceExtensions {
                ..Default::default()
            },
            enumerate_portability: true,
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
