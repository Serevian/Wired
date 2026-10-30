use anyhow::Result;
use ash::vk;

use crate::engine::{buffer::Buffer, device::Device, shader_data::ShaderData};

pub struct Frame {
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

        Ok(Self {
            shader_data_buffer: buffer,
        })
    }

    pub fn destroy(&mut self, allocator: &vk_mem::Allocator) {
        self.shader_data_buffer.destroy(allocator);
    }
}
