use std::sync::Arc;

use thiserror::Error;
use vulkano::{
    device::Device,
    image::{
        view::{ImageView, ImageViewCreationError},
        ImageAccess, SwapchainImage,
    },
    pipeline::graphics::viewport::Viewport,
    render_pass::{
        Framebuffer, FramebufferCreateInfo, FramebufferCreationError, RenderPass,
        RenderPassCreationError,
    },
    swapchain::Swapchain,
};


pub struct VulkanRenderPass {
    render_pass: Arc<RenderPass>,
}

impl VulkanRenderPass {
    pub fn new(
        device: Arc<Device>,
        swap_chain: Arc<Swapchain>,
        images: &[Arc<SwapchainImage>],
        viewport: &mut Viewport,
    ) -> Result<(Self, Vec<Arc<Framebuffer>>), CreateVulkanRenderPassError> {
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

        let dimensions = images[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

        let mut framebuffers = vec![];

        for image in images {
            let view = ImageView::new_default(image.clone())?;
            framebuffers.push(Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )?)
        }

        Ok((Self { render_pass }, framebuffers))
    }
}

#[derive(Debug, Error)]
pub enum CreateVulkanRenderPassError {
    #[error("failed to create render pass")]
    CreateRenderPassError(#[from] RenderPassCreationError),
    #[error("failed to create an image view")]
    ImageViewCreationError(#[from] ImageViewCreationError),
    #[error("failed to create a framebuffer")]
    FramebufferCreationError(#[from] FramebufferCreationError),
}
