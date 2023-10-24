use std::error::Error;

use vulkan_renderer::{
    init::{VulkanConfig, VulkanInit},
    swapchain::VulkanSwapChain,
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

    let swapchain = VulkanSwapChain::new_with_init(&init, surface.clone(), &window)?;
    event_loop.run(event_loop_handler)
}

fn event_loop_handler<T>(
    event: Event<T>,
    _event_loop_window_target: &EventLoopWindowTarget<T>,
    control_flow: &mut ControlFlow,
) {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        _ => (),
    }
}
