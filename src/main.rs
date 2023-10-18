use std::error::Error;

use sdl2::event::Event;
use vulkan_renderer::init::VulkanInit;
use vulkano::instance::InstanceExtensions;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window("Vulkan Renderer Test", 800, 600)
        .vulkan()
        .build()?;

    let instance_extensions =
        InstanceExtensions::from_iter(window.vulkan_instance_extensions()?);

    let _vulkan_init = VulkanInit::new(instance_extensions)?;

    let mut event_pump = sdl.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {
                }
            }
        }
    }
    Ok(())
}
