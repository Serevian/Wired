use anyhow::Result;
use ash::vk;

use crate::engine::{buffer::Buffer, device::Device, shader_data::ShaderData};

pub struct Frame {
    fence: vk::Fence,
    image_acquired_semaphore: vk::Semaphore,
    shader_data_buffer: Buffer,
}

impl Frame {
    pub fn new(device: &Device, allocator: &vk_mem::Allocator) -> Result<Self> {
        let buffer = Buffer::create_device_local(
            device,
            allocator,
            size_of::<ShaderData>() as u64,
            vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        )?;

        let fence_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        let fence = unsafe { device.logical.create_fence(&fence_info, None)? };

        let sempahore_info = vk::SemaphoreCreateInfo::default();
        let image_acquired_semaphore =
            unsafe { device.logical.create_semaphore(&sempahore_info, None)? };

        Ok(Self {
            fence,
            image_acquired_semaphore,
            shader_data_buffer: buffer,
        })
    }

    pub fn destroy(&mut self, device: &Device, allocator: &vk_mem::Allocator) {
        unsafe {
            device.logical.destroy_fence(self.fence, None);
            device
                .logical
                .destroy_semaphore(self.image_acquired_semaphore, None);
        }
        self.shader_data_buffer.destroy(allocator);
    }
}
