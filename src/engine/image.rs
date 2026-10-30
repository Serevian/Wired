use anyhow::Result;
use ash::vk::{self, Extent3D, Format};
use vk_mem::Alloc;

use crate::engine::device::Device;

pub struct Image {
    view: vk::ImageView,
    allocation: vk_mem::Allocation,
    raw: vk::Image,
    pub extent: Extent3D,
    pub format: Format,
}

impl Image {
    pub fn depth(
        device: &Device,
        allocator: &vk_mem::Allocator,
        extent: vk::Extent3D,
    ) -> Result<Self> {
        let format = vk::Format::D32_SFLOAT_S8_UINT;
        let create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(extent)
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .initial_layout(vk::ImageLayout::UNDEFINED);

        let allocation_info = vk_mem::AllocationCreateInfo {
            flags: vk_mem::AllocationCreateFlags::DEDICATED_MEMORY,
            usage: vk_mem::MemoryUsage::Auto,
            ..Default::default()
        };

        let (image, allocation) =
            unsafe { allocator.create_image(&create_info, &allocation_info)? };

        let view_create_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH)
                    .level_count(1)
                    .layer_count(1),
            );

        let view = unsafe { device.logical.create_image_view(&view_create_info, None)? };

        Ok(Self {
            view,
            allocation,
            raw: image,
            extent,
            format,
        })
    }

    pub fn destroy(&mut self, device: &Device, allocator: &vk_mem::Allocator) {
        unsafe {
            device.logical.destroy_image_view(self.view, None);
            allocator.destroy_image(self.raw, &mut self.allocation);
        }
    }
}
