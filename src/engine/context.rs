use std::ffi::{CStr, c_void};

use anyhow::Result;
use ash::vk::{self, TaggedStructure};
use raw_window_handle::RawDisplayHandle;

const INSTANCE_EXTENSIONS: [&CStr; 1] = [ash::ext::debug_utils::NAME];
const VALIDATION_LAYERS: [&CStr; 1] = [c"VK_LAYER_KHRONOS_validation"];

pub struct Context {
    pub instance: ash::Instance,
    debug_utils: ash::ext::debug_utils::Instance,
    messenger: vk::DebugUtilsMessengerEXT,
    _entry: ash::Entry,
}

impl Context {
    pub fn new(display_handle: &RawDisplayHandle) -> Result<Self> {
        let entry = unsafe { ash::Entry::load()? };

        let application_info = vk::ApplicationInfo::default()
            .application_name(c"Wired")
            .api_version(vk::API_VERSION_1_4);

        // I should really check if extensions and layers are use
        let mut instance_extensions: Vec<&CStr> =
            ash_window::enumerate_required_extensions(*display_handle)?
                .iter()
                .map(|ptr| unsafe { CStr::from_ptr(*ptr) })
                .collect();
        instance_extensions.extend(INSTANCE_EXTENSIONS.iter());
        instance_extensions.sort();

        tracing::info!("Enabled instance extensions:");
        for ext in &instance_extensions {
            tracing::info!("\t{ext:?}");
        }

        let instance_extensions_ptrs: Vec<*const i8> =
            instance_extensions.iter().map(|cs| cs.as_ptr()).collect();

        let validation_layers_ptrs: Vec<*const i8> =
            VALIDATION_LAYERS.iter().map(|cs| cs.as_ptr()).collect();

        let mut debug_info = Self::debug_messenger_info();

        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_extension_names(&instance_extensions_ptrs)
            .enabled_layer_names(&validation_layers_ptrs)
            .push(&mut debug_info);

        let instance = unsafe { entry.create_instance(&instance_info, None)? };

        let debug_utils = ash::ext::debug_utils::Instance::load(&entry, &instance);
        let messenger = unsafe {
            debug_utils.create_debug_utils_messenger(&Self::debug_messenger_info(), None)?
        };

        Ok(Self {
            instance,
            debug_utils,
            messenger,
            _entry: entry,
        })
    }

    fn debug_messenger_info<'a>() -> vk::DebugUtilsMessengerCreateInfoEXT<'a> {
        vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(Self::vulkan_debug_callback))
    }

    unsafe extern "system" fn vulkan_debug_callback(
        severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        msg_type: vk::DebugUtilsMessageTypeFlagsEXT,
        data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
        _user_data: *mut c_void,
    ) -> vk::Bool32 {
        let data = unsafe { *data };
        let id_name = unsafe {
            CStr::from_ptr(data.p_message_id_name)
                .to_string_lossy()
                .to_string()
        };
        let message = unsafe { CStr::from_ptr(data.p_message).to_string_lossy().to_string() };
        let id_number = data.message_id_number;

        if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
            tracing::error!("[{msg_type:?}] [{id_name}-{id_number:#x}]: {message}");
        } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
            tracing::warn!("[{msg_type:?}] [{id_name}-{id_number:#x}]: {message}");
        } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::INFO) {
            tracing::info!("[{msg_type:?}] [{id_name}-{id_number:#x}]: {message}");
        } else {
            tracing::trace!("[{msg_type:?}] [{id_name}-{id_number:#x}]: {message}");
        }

        vk::FALSE
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils
                .destroy_debug_utils_messenger(self.messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}
