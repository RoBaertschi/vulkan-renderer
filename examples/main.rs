use std::error::Error;

use vulkan_renderer::{
    init::{VulkanConfig, VulkanInit},
    renderpass::VulkanRenderPass,
    swapchain::VulkanSwapChain,
};
use vulkano::pipeline::graphics::viewport::Viewport;
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

    let swap_chain = VulkanSwapChain::new_with_init(&init, surface.clone(), &window)?;
    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    let render_pass = VulkanRenderPass::new(
        init.device.clone(),
        swap_chain.swap_chain.clone(),
        &swap_chain.images,
        &mut viewport,
    );

    event_loop.run(
        |event: Event<_>,
         _event_loop_window_target: &EventLoopWindowTarget<_>,
         control_flow: &mut ControlFlow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            }
        },
    )
}
