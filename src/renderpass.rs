use std::sync::Arc;

use thiserror::Error;
use vulkano::{
    device::Device,
    render_pass::{RenderPass, RenderPassCreationError},
    swapchain::Swapchain, pipeline::graphics::viewport::Viewport,
};

pub struct VulkanRenderPass {
    render_pass: Arc<RenderPass>,
}

impl VulkanRenderPass {
    pub fn new(
        device: Arc<Device>,
        swap_chain: Arc<Swapchain>,
        _viewport: Viewport,
    ) -> Result<Self, CreateVulkanRenderPassError> {
        let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swap_chain.image_format(),
                samples: 1,
            }
        },
        pass : {
            color: [color],
            depth_stencil: {}
        }
        )?;

        Ok(Self { render_pass })
    }
}

#[derive(Debug, Error)]
pub enum CreateVulkanRenderPassError {
    #[error("failed to create render pass")]
    CreateRenderPassError(#[from] RenderPassCreationError),
}
