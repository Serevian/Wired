use anyhow::Result;
use ash::{ext, vk};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::engine::{context::Context, device::Device};

pub struct Surface {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub raw: vk::SurfaceKHR,
    loader: ash::khr::surface::Instance,
}

impl Surface {
    pub fn new(
        context: &Context,
        device: &Device,
        display_handle: &RawDisplayHandle,
        window_handle: &RawWindowHandle,
    ) -> Result<Self> {
        let loader = ash::khr::surface::Instance::load(&context.entry, &context.instance);

        let surface = unsafe {
            ash_window::SurfaceFactory::new(&context.entry, &context.instance, *display_handle)?
                .create_surface(*window_handle, None)?
        };

        let get_capabilities =
            ash::khr::get_surface_capabilities2::Instance::load(&context.entry, &context.instance);

        let capabilities =
            unsafe { loader.get_physical_device_surface_capabilities(device.physical, surface)? };

        Ok(Self {
            capabilities,
            raw: surface,
            loader,
        })
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_surface(self.raw, None);
        }
    }
}
