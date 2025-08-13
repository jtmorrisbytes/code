#![deny(unused_must_use)]
// VK_EXT_ helper struct

use ash::vk::{
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCreateFlagsEXT,
};

// ash::instance helper struct for automatic cleanup
pub struct VkInstance(ash::Instance);
impl VkInstance {
    pub fn new(i: ash::Instance) -> Self {
        Self(i)
    }
}
impl std::ops::Deref for VkInstance {
    type Target = ash::Instance;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Drop for VkInstance {
    fn drop(&mut self) {
        unsafe { self.0.destroy_instance(None) };
    }
}

// ash::Device helper struct for automatic cleanup

pub struct VkDevice(ash::Device);
impl VkDevice {
    pub fn new(d: ash::Device) -> Self {
        Self(d)
    }
}
impl std::ops::Deref for VkDevice {
    type Target = ash::Device;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Drop for VkDevice {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy_device(None);
        }
    }
}

pub struct VkExtDebugUtils {
    function_loader: ash::ext::debug_utils::Instance,
    // create_info: ash::vk::DebugUtilsMessengerCreateInfoEXT,
    instance: ash::vk::DebugUtilsMessengerEXT,
}
/// an example debug callback function that prints all messages to STDOUT
#[unsafe(no_mangle)]
pub unsafe extern "system" fn _____default_debug_callback_fn(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _p_user_data: *mut std::ffi::c_void,
) -> ash::vk::Bool32 {
    if p_callback_data.is_null() {
        return 0;
    }
    let callback_data = unsafe {*p_callback_data};
    let message = unsafe {callback_data.message_id_name_as_c_str()};
    println!(
        "SEVERITY: {message_severity:?}, MESSAGE_TYPES: {message_types:?}, MESSAGE: {message:?}"
    );
    return 0;
}
/// Creates and manages an instance of the VK_EXT_debug utils extension. ash::Intance and ash::Entry structs are required
/// Cleans itself up when dropped
impl VkExtDebugUtils {
    pub fn try_new(
        entry: &ash::Entry,
        instance: &ash::Instance,
        callback_fn: ash::vk::PFN_vkDebugUtilsMessengerCallbackEXT,
    ) -> anyhow::Result<Self> {
        let function_loader = ash::ext::debug_utils::Instance::new(&entry, &instance);
        let create_info = ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
            .pfn_user_callback(callback_fn)
            .flags(DebugUtilsMessengerCreateFlagsEXT::default())
            .message_severity(
                DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            );
        let instance = unsafe { function_loader.create_debug_utils_messenger(&create_info, None) }?;
        Ok(Self {
            function_loader,
            instance,
        })
    }
}
impl Drop for VkExtDebugUtils {
    fn drop(&mut self) {
        unsafe {
            self.function_loader
                .destroy_debug_utils_messenger(self.instance, None)
        };
    }
}
