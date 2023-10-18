use std::error::Error;

use vulkan_renderer::init::VulkanInit;
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

fn main() -> Result<(), Box<dyn Error>> {
    let init = VulkanInit::new_winit()?;

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new().build_vk_surface(&event_loop, init.instance.clone())?;

    let _window = surface
        .object()
        .unwrap()
        .clone()
        .downcast::<Window>()
        .unwrap();

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
