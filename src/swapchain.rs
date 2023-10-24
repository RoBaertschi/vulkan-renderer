use std::sync::Arc;

use thiserror::Error;
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceError},
        Device,
    },
    image::SwapchainImage,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo, SwapchainCreationError},
};
use winit::window::Window;

use crate::init::VulkanInit;

pub struct VulkanSwapChain {
    pub swap_chain: Arc<Swapchain>,
    pub images: Vec<Arc<SwapchainImage>>,
}

impl VulkanSwapChain {
    pub fn new(
        physical_device: Arc<PhysicalDevice>,
        device: Arc<Device>,
        surface: Arc<Surface>,
        window: &Window,
    ) -> Result<Self, CreateVulkanSwapChainError> {
        let caps = physical_device.surface_capabilities(&surface, Default::default())?;
        let dimensions = window.inner_size();
        let usage = caps.supported_usage_flags;
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = Some(
            physical_device
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0,
        );

        let (swap_chain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count + 1,
                image_format,
                image_extent: dimensions.into(),
                image_usage: usage,
                composite_alpha,
                ..Default::default()
            },
        )?;

        Ok(Self { swap_chain, images })
    }

    pub fn new_with_init(
        vulkan_init: &VulkanInit,
        surface: Arc<Surface>,
        window: &Window,
    ) -> Result<Self, CreateVulkanSwapChainError> {
        Self::new(
            vulkan_init.physical_device.clone(),
            vulkan_init.device.clone(),
            surface.clone(),
            window,
        )
    }
}

#[derive(Debug, Error)]
pub enum CreateVulkanSwapChainError {
    #[error("could not get surface capabilities")]
    PhysicalDeviceError(#[from] PhysicalDeviceError),
    #[error("could not create swap chain")]
    CreateSwapChainError(#[from] SwapchainCreationError),
}
