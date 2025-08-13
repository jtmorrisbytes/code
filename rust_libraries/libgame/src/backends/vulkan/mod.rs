use crate::{
    backends::vulkan::helper_structs::{VkDevice, VkExtDebugUtils, _____default_debug_callback_fn},
    engine::backend::{GraphicsBackend, RenderTarget},
};
use anyhow::Context;
use ash::vk::{self, QueueFlags};
pub mod helper_structs;
// Drop::drop is called in order of the fields.
// ash::Entry must be last, then the instance second last,
// then any other objects that depend on instance must be declared
// in reverse order that they are
pub struct VulkanBackend {
    device_graphics_queue: ash::vk::Queue,
    device: VkDevice,
    debug_utils: Option<helper_structs::VkExtDebugUtils>,
    instance: helper_structs::VkInstance,
    entry: ash::Entry,
}

pub struct InstanceCreateOptions<EngineName, ApplicationName>
where
    EngineName: AsRef<str>,
    ApplicationName: AsRef<str>,
{
    pub engine_name: EngineName,
    pub application_name: ApplicationName,
    pub headless: bool,
    pub enable_validation_layers: bool,
    pub enable_debug_logging: bool,
}

fn create_instance<S: AsRef<str>>(
    entry: &ash::Entry,
    options: InstanceCreateOptions<S, S>,
) -> anyhow::Result<ash::Instance> {
    let app_name = std::ffi::CString::new(options.application_name.as_ref())?;
    let engine_name = std::ffi::CString::new(options.engine_name.as_ref())?;

    let appinfo = ash::vk::ApplicationInfo::default()
        .application_name(&app_name)
        .engine_name(&engine_name)
        .api_version(vk::API_VERSION_1_2)
        .engine_version(0)
        .application_version(0);
    let mut enabled_instance_extensions = vec![];
    let mut enabled_validation_layers = vec![];
    let available_extensions =
        unsafe { entry.enumerate_instance_extension_properties(None) }.unwrap_or_default();

    if options.headless == false {
        let mut requested_extensions: Vec<_> = available_extensions
            .iter()
            .filter_map(|extension_properties| {
                // enable VK_KHR_SURFACE extenstion if the name matches and the spec version is greater or equal to the spec version (I assume the verion ash supports)
                let extension_name_cstr = extension_properties.extension_name_as_c_str().unwrap();
                if extension_name_cstr == ash::khr::surface::NAME
                    && extension_properties.spec_version >= ash::khr::surface::SPEC_VERSION
                {
                    println!("enabling VK_KHR_surface");
                    return Some(ash::khr::surface::NAME.as_ptr());
                }
                // VK_KHR_SWAPCHAIN
                if extension_name_cstr == ash::khr::swapchain::NAME
                    && extension_properties.spec_version >= ash::khr::swapchain::SPEC_VERSION
                {
                    println!("enabling VK_KHR_swapchain");

                    return Some(ash::khr::swapchain::NAME.as_ptr());
                }
                #[cfg(target_os = "windows")]
                {
                    if extension_name_cstr == ash::khr::win32_surface::NAME
                        && extension_properties.spec_version
                            >= ash::khr::win32_surface::SPEC_VERSION
                    {
                        println!("enabling VK_KHR_win32_surface");
                        return Some(ash::khr::win32_surface::NAME.as_ptr());
                    }
                }
                #[cfg(target_os = "linux")]
                {
                    if extension_name_cstr == ash::khr::xcb_surface::NAME
                        && extension_properties.spec_version >= ash::khr::xcb_surface::SPEC_VERSION
                    {
                        println!("enabling VK_KHR_xcb_surface");
                        return Some(ash::khr::xcb_surface::NAME);
                    }
                    if extension_name_cstr == ash::khr::xlib_surface::NAME
                        && extension_properties.spec_version >= ash::khr::xlib_surface::SPEC_VERSION
                    {
                        println!("enabling VK_KHR_xlib_surface")
                    }
                }
                return None;
            })
            .collect();
        enabled_instance_extensions.append(&mut requested_extensions);
    }
    if options.enable_validation_layers {
        if let Some(VK_EXT_debug_utils_name) = available_extensions
            .iter()
            .find(|extension_properties| {
                extension_properties.extension_name_as_c_str().unwrap()
                    == ash::ext::debug_utils::NAME
                    && extension_properties.spec_version >= ash::ext::debug_utils::SPEC_VERSION
            })
            .map(|_| ash::ext::debug_utils::NAME.as_ptr())
        {
            enabled_instance_extensions.push(VK_EXT_debug_utils_name);
            // enable the debug laer
        }
        if unsafe { entry.enumerate_instance_layer_properties() }?
            .iter()
            .find(|l_p| l_p.layer_name_as_c_str().unwrap() == c"VK_LAYER_KHRONOS_validation")
            .is_some()
        {
            enabled_validation_layers.push(c"VK_LAYER_KHRONOS_validation".as_ptr())
        }
    }
    let mut create_info = ash::vk::InstanceCreateInfo::default()
        .application_info(&appinfo)
        .enabled_extension_names(&enabled_instance_extensions)
        .enabled_layer_names(&enabled_validation_layers);

    let instance = unsafe { entry.create_instance(&create_info, None) }?;
    Ok(instance)
}

impl GraphicsBackend for VulkanBackend {
    fn init(backend_options: &crate::engine::backend::BackendOptions) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let entry = unsafe { ash::Entry::load() }?;

        let instance_create_options = InstanceCreateOptions {
            engine_name: "Jordan's Basic vulkan engine",
            application_name: "To be inserted later",
            headless: backend_options.render_target == RenderTarget::Headless,
            enable_validation_layers: backend_options.enable_debug_logging,
            enable_debug_logging: backend_options.enable_debug_logging,
        };
        let instance = create_instance(&entry, instance_create_options)?;
        // wrap the raw instance in a helper struct because we want to clean up automatically at the end of this function
        let instance = helper_structs::VkInstance::new(instance);
        // enable the debug utils here if neccessary
        // create an instance of the debug utils function loader

        let debug_utils = {
            if backend_options.enable_debug_logging {
                Some(VkExtDebugUtils::try_new(
                    &entry,
                    &instance,
                    Some(_____default_debug_callback_fn),
                )?)
            } else {
                None
            }
        };
        // -- pick physical devices --

        // enumerate the devices on the system
        let physical_devices = unsafe { instance.enumerate_physical_devices() }
            .context("Vulkan failed to enumerate physical devices (GPUS)")?;
        if physical_devices.len() == 0 {
            return Err(anyhow::Error::msg(
                "Vulkan failed to find any supported physical devices on the system.",
            ));
        }

        let mut chosen_physical_device = None;
        for physical_device in physical_devices {
            let properties = unsafe { instance.get_physical_device_properties(physical_device) };

            let features = unsafe { instance.get_physical_device_features(physical_device) };

            println!("{properties:?}\n");
            println!("{features:?}");
            chosen_physical_device = Some(physical_device);
        }
        if chosen_physical_device.is_none() {
            return Err(anyhow::Error::msg(
                "Vulkan: There is no physical device suitable for this application",
            ));
        }
        let physical_device = chosen_physical_device.unwrap();

        // find queue families

        let queue_family_index = match find_graphics_queue_indicies(&instance, physical_device) {
            Ok(value) => value,
            Err(value) => return value,
        };


        
        // now we need to create queues
        let mut queue_create_info = ash::vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index as u32)
            .queue_priorities(&[1.0_f32]);
        let required_device_features = ash::vk::PhysicalDeviceFeatures::default();

        let infos = [queue_create_info];
        let device_create_info = ash::vk::DeviceCreateInfo::default()
            .enabled_features(&required_device_features)
            .queue_create_infos(&infos);

        let logical_device = unsafe {instance.create_device(physical_device, &device_create_info, None)}?;
        // use the hepler struct that calls drop automatially
        let logical_device = VkDevice::new(logical_device);

        // get a reference to the queue we just created
        let device_graphics_queue = unsafe {logical_device.get_device_queue(queue_family_index as u32, 0)};
        Ok(Self {
            device_graphics_queue,
            device:logical_device,
            debug_utils,
            entry,
            instance,
        })
    }
    fn render_frame(&mut self, scene: ()) {}
    fn resize(&mut self, width: u32, height: u32) {}
    fn resume(&mut self, options: &crate::engine::backend::BackendOptions) {}
    fn shutdown(&mut self) {}
    fn suspend(&mut self) {}
}

fn find_graphics_queue_indicies(instance: &ash::Instance, physical_device: vk::PhysicalDevice,headless:bool) -> Result<usize, Result<VulkanBackend, anyhow::Error>> {
    let queue_family_properties =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let mut graphics_queue = None;
    for (index, properties) in queue_family_properties.iter().enumerate() {
        if properties.queue_flags.contains(QueueFlags::GRAPHICS) {
            graphics_queue = Some((index, *properties));
            break;
        }
    }
    if graphics_queue.is_none() {
        return Err(Err(anyhow::Error::msg(
            "Failed to find a sutable GRAPHICS queue",
        )));
    }
    let (queue_family_index, queue_family_poperties) =
        graphics_queue.unwrap();
    Ok(queue_family_index)
}
