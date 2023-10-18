use std::sync::Arc;

use thiserror::Error;
use vulkano::{instance::Instance, VulkanObject, Handle, swapchain::{Surface, SurfaceApi}};

fn create_surface_and_handle(window: &sdl2::video::Window, instance: Arc<Instance>) -> Result<(), CreateSurfaceAndHandleError> {
    let surface_handle = window.vulkan_create_surface(
        instance .handle().as_raw() as _
    )?;

    let surface = unsafe {
        Surface::from_handle(instance.clone(), <_ as Handle>::from_raw(surface_handle), SurfaceApi::, object)

    }

    Ok(())
}

fn get_surface_api() -> SurfaceApi {
    
}

#[derive(Debug, Error)]
enum CreateSurfaceAndHandleError {
    #[error(transparent)]
    CreateSurfaceHandleError(#[from] SDLError),
    
}

impl From<String> for CreateSurfaceAndHandleError {
    fn from(value: String) -> Self {
        Self::CreateSurfaceHandleError(value.into())    
    }
}

#[derive(Debug, Error)]
#[error("SDLError: {}", .0)]
struct SDLError(String);

impl From<String> for SDLError {
    fn from(value: String) -> Self {
        Self(value)
    }
}
