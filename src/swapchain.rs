use std::sync::Arc;

use thiserror::Error;
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceError},
        Device,
    },
    image::SwapchainImage,
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, RenderPass},
    swapchain::{Surface, Swapchain, SwapchainCreateInfo, SwapchainCreationError},
};
use winit::window::Window;

use crate::{init::VulkanInit, render_pass::{window_size_dependent_setup, WindowSizeDependentSetupError}};

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
    #[cfg(feature = "winit")]
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
    /// Returns, if the swap chain got recreated.
    /// panics: If a unknown error occured
    pub fn  recreate<'a>(
        &mut self,
        image_extent: [u32; 2],
        framebuffers: &'a mut Vec<Arc<Framebuffer>>,
        viewport: &mut Viewport,
        render_pass: Arc<RenderPass>,
    ) -> Result<bool, RecreateVulkanSwapChainError> {
        let (new_swapchain, new_images) = match self.swap_chain.recreate(SwapchainCreateInfo {
            image_extent,
            ..self.swap_chain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageUsageNotSupported { .. }) => return Ok(false),
            Err(e) => panic!("failed to recreate swapchain: {:?}", e),
        };

        self.swap_chain = new_swapchain;
        framebuffers.clear();

        framebuffers.append(
            &mut window_size_dependent_setup(&new_images.clone(), render_pass.clone(), viewport)?
        );
        return Ok(true);
    }
}

#[derive(Debug, Error)]
pub enum RecreateVulkanSwapChainError {
    #[error("failed to setup the framebuffers")]
    WindowSizeDependentSetupError(#[from] WindowSizeDependentSetupError)
}

#[derive(Debug, Error)]
pub enum CreateVulkanSwapChainError {
    #[error("could not get surface capabilities")]
    PhysicalDeviceError(#[from] PhysicalDeviceError),
    #[error("could not create swap chain")]
    CreateSwapChainError(#[from] SwapchainCreationError),
}
