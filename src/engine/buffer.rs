use anyhow::Result;
use ash::vk;
use vk_mem::Alloc;

use crate::engine::device::Device;

pub struct Buffer {
    pub handle: vk::Buffer,
    pub allocation: vk_mem::Allocation,
    pub size: vk::DeviceSize,
    pub address: vk::DeviceAddress,
}

impl Buffer {
    fn new(
        device: &Device,
        allocator: &vk_mem::Allocator,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        alloc_info: &vk_mem::AllocationCreateInfo,
    ) -> Result<Self> {
        let info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let (handle, allocation) = unsafe { allocator.create_buffer(&info, alloc_info)? };

        let address = if usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS) {
            let address_info = vk::BufferDeviceAddressInfo::default().buffer(handle);
            unsafe { device.logical.get_buffer_device_address(&address_info) }
        } else {
            0
        };

        Ok(Self {
            handle,
            allocation,
            size,
            address,
        })
    }

    pub fn create_device_local(
        device: &Device,
        allocator: &vk_mem::Allocator,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
    ) -> Result<Self> {
        let alloc_info = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::AutoPreferDevice,
            flags: vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE
                | vk_mem::AllocationCreateFlags::MAPPED
                | vk_mem::AllocationCreateFlags::HOST_ACCESS_ALLOW_TRANSFER_INSTEAD,
            ..Default::default()
        };

        let buf = Self::new(device, allocator, size, usage, &alloc_info)?;

        Ok(buf)
    }

    // REBAR Buffer
    pub fn create_device_local_and_write(
        device: &Device,
        allocator: &vk_mem::Allocator,
        data: &[u8],
        usage: vk::BufferUsageFlags,
    ) -> Result<Self> {
        let buf =
            Self::create_device_local(device, allocator, data.len() as vk::DeviceSize, usage)?;

        unsafe {
            let info = allocator.get_allocation_info(&buf.allocation);
            std::ptr::copy_nonoverlapping(data.as_ptr(), info.mapped_data as *mut u8, data.len());
            allocator.flush_allocation(&buf.allocation, 0, vk::WHOLE_SIZE)?;
        }

        Ok(buf)
    }

    pub fn destroy(&mut self, allocator: &vk_mem::Allocator) {
        unsafe {
            allocator.destroy_buffer(self.handle, &mut self.allocation);
        }
    }
}
