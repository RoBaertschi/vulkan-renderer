use vulkano::swapchain::SurfaceApi;

#[cfg(all(unix, feature = "wayland"))]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Wayland;
}

#[cfg(all(unix, feature = "x11"))]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Xlib;
}

#[cfg(windows)]
fn get_surface_api() -> SurfaceApi {
    use vulkano::swapchain::SurfaceApi;

    return SurfaceApi::Win32;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Metal;
}


#[cfg(target_os = "android")]
fn get_surface_api() -> SurfaceApi {
    return SurfaceApi::Android;
}

