use std::ffi::CStr;

use anyhow::{Context, Result};
use ash::vk::{self, TaggedStructure};

const DEVICE_EXTENSIONS: [&CStr; 1] = [ash::khr::swapchain::NAME];

pub struct Queue {
    raw: vk::Queue,
    pub family_index: u32,
    pub index: u32,
}

pub struct Device {
    pub logical: ash::Device,
    pub physical: vk::PhysicalDevice,
}

impl Device {
    pub fn new(context: &crate::engine::context::Context) -> Result<(Self, Queue)> {
        let physical_devices = unsafe { context.instance.enumerate_physical_devices()? };
        // Maybe do a better device search?
        let physical_device = physical_devices[0];
        let mut device_properties = vk::PhysicalDeviceProperties2::default();
        unsafe {
            context
                .instance
                .get_physical_device_properties2(physical_device, &mut device_properties);
        }
        let physical_name = device_properties.properties.device_name_as_c_str()?;
        tracing::info!("Chosen device: {physical_name:?}");

        let queue_families_len = unsafe {
            context
                .instance
                .get_physical_device_queue_family_properties2_len(physical_device)
        };
        let mut queue_families = vec![vk::QueueFamilyProperties2::default(); queue_families_len];
        unsafe {
            context
                .instance
                .get_physical_device_queue_family_properties2(physical_device, &mut queue_families);
        }
        let (queue_family_index, _) = queue_families
            .iter()
            .enumerate()
            .find(|(_, qfp)| {
                qfp.queue_family_properties
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS)
            })
            .context("Error finding a suitable queue family")?;
        let queue_family_index = u32::try_from(queue_family_index)?;

        let queue_create_info = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0])];

        tracing::info!("Enabled device extensions:");
        for ext in &DEVICE_EXTENSIONS {
            tracing::info!("\t{ext:?}");
        }

        let device_extensions_ptrs: Vec<*const i8> =
            DEVICE_EXTENSIONS.iter().map(|cs| cs.as_ptr()).collect();

        let vulkan_1_0_features = vk::PhysicalDeviceFeatures::default().sampler_anisotropy(true);
        let mut vulkan_1_2_features = vk::PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true)
            .descriptor_binding_variable_descriptor_count(true)
            .descriptor_indexing(true)
            .runtime_descriptor_array(true)
            .shader_sampled_image_array_non_uniform_indexing(true);
        let mut vulkan_1_3_features = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_info)
            .enabled_extension_names(&device_extensions_ptrs)
            .enabled_features(&vulkan_1_0_features)
            .push(&mut vulkan_1_2_features)
            .push(&mut vulkan_1_3_features);

        let device = unsafe {
            context
                .instance
                .create_device(physical_device, &device_create_info, None)?
        };

        let raw_queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let queue = Queue {
            raw: raw_queue,
            family_index: queue_family_index,
            index: 0,
        };

        Ok((
            Self {
                logical: device,
                physical: physical_device,
            },
            queue,
        ))
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.logical.destroy_device(None);
        }
    }
}
