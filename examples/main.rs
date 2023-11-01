use std::error::Error;

use vulkan_renderer::{
    init::{VulkanConfig, VulkanInit},
    render_pass::VulkanRenderPass,
    swapchain::VulkanSwapChain,
};
use vulkano::{
    pipeline::graphics::viewport::Viewport,
    swapchain::{SwapchainCreateInfo, SwapchainCreationError, self, AcquireError},
    sync::{self, GpuFuture},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::Window,
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let (init, surface) = VulkanInit::new_winit(
        VulkanConfig {
            swap_chain: true,
            ..Default::default()
        },
        &event_loop,
    )?;

    let window = surface
        .object()
        .unwrap()
        .clone()
        .downcast::<Window>()
        .unwrap();

    let mut swap_chain = VulkanSwapChain::new_with_init(&init, surface.clone(), &window)?;
    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let render_pass = VulkanRenderPass::new(init.device.clone(), swap_chain.swap_chain.clone())?;

    let mut framebuffers = vulkan_renderer::render_pass::window_size_dependent_setup(
        &swap_chain.images.clone(),
        render_pass.render_pass.clone(),
        &mut viewport,
    )?;

    let mut recreate_swap_chain = false;
    let mut previous_frame_end =
        Some(Box::new(sync::now(init.device.clone())) as Box<dyn GpuFuture>);

    event_loop.run(
        move |event: Event<_>,
              _event_loop_window_target: &EventLoopWindowTarget<_>,
              control_flow: &mut ControlFlow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    recreate_swap_chain = true;
                }
                Event::RedrawEventsCleared => {
                    previous_frame_end
                        .as_mut()
                        .take()
                        .unwrap()
                        .cleanup_finished();
                    // Do some drawing
                    //

                    let (image_index, suboptimal, aquire_future) = match swapchain::acquire_next_image(swap_chain.swap_chain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swap_chain = true;
                            return;
                        },
                        Err(e) => panic!("Failed to acquire next image: {:?}", e),
                    };

                    if suboptimal {
                        recreate_swap_chain = true;
                    }

                    if recreate_swap_chain {
                        let image_extent: [u32; 2] = window.inner_size().into();

                        recreate_swap_chain = !swap_chain.recreate(
                            image_extent,
                            &mut framebuffers,
                            &mut viewport,
                            render_pass.render_pass.clone(),
                        ).unwrap();
                    }
                }
                _ => (),
            }
        },
    )
}
