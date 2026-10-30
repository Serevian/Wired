use anyhow::Result;
use ash::vk::{self, Extent2D, Image, SwapchainKHR};

use crate::engine::{context::Context, device::Device, surface::Surface};

pub struct Swapchain {
    pub images: Vec<Image>,
    pub size: Extent2D,
    pub raw: SwapchainKHR,
    loader: ash::khr::swapchain::Device,
}

impl Swapchain {
    pub fn new(
        context: &Context,
        device: &Device,
        surface: &Surface,
        window_size: (u32, u32),
    ) -> Result<Self> {
        let loader = ash::khr::swapchain::Device::load(&context.instance, &device.logical);

        // Wayland compositors in Linux behave strangely, giving a 0xFFFF_FFFF width
        let swapchain_extent = if surface.capabilities.current_extent.width == 0xFFFF_FFFF {
            vk::Extent2D::default()
                .width(window_size.0)
                .height(window_size.1)
        } else {
            surface.capabilities.current_extent
        };
        let swapchain_format = vk::Format::B8G8R8A8_SRGB;
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.raw)
            .min_image_count(surface.capabilities.min_image_count)
            .image_format(swapchain_format)
            .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
            .image_extent(swapchain_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO);

        tracing::info!("Creating swapchain with:");
        tracing::info!("\tFormat: {swapchain_format:?}");
        tracing::info!(
            "\tMin Image Count: {}",
            surface.capabilities.min_image_count
        );
        tracing::info!("\tExtent: {swapchain_extent:?}");

        let swapchain = unsafe { loader.create_swapchain(&swapchain_create_info, None)? };

        let images = unsafe { loader.get_swapchain_images(swapchain)? };

        Ok(Self {
            images,
            size: swapchain_extent,
            raw: swapchain,
            loader,
        })
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_swapchain(self.raw, None);
        }
    }
}
