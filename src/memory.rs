use std::sync::Arc;

use vulkano::{command_buffer::allocator::StandardCommandBufferAllocator, device::Device};

pub struct Allocators {
    standart: StandardCommandBufferAllocator,
}

impl Allocators {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            standart: StandardCommandBufferAllocator::new(device.clone(), Default::default()),
        }
    }
}
