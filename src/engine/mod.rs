use anyhow::{Ok, Result};
use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::engine::{
    context::Context,
    device::{Device, Queue},
};

mod context;
mod device;

pub struct Engine {
    allocator: vk_mem::Allocator,
    queue: Queue,
    device: Device,
    context: Context,
}

impl Engine {
    pub fn new(display_handle: RawDisplayHandle, window_handle: RawWindowHandle) -> Result<Self> {
        let context = Context::new(&display_handle)?;
        let (device, queue) = Device::new(&context)?;

        let mut allocator_info =
            vk_mem::AllocatorCreateInfo::new(&context.instance, &device.logical, device.physical);
        allocator_info.vulkan_api_version = vk::API_VERSION_1_4;
        allocator_info.flags = vk_mem::AllocatorCreateFlags::BUFFER_DEVICE_ADDRESS;
        let allocator = unsafe { vk_mem::Allocator::new(allocator_info)? };

        Ok(Self {
            allocator,
            queue,
            device,
            context,
        })
    }
}
