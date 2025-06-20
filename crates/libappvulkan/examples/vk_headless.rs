pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    use libappvulkan::Loader;
    let loader = Loader::try_new()?;

    let app_info = libappvulkan::bindings::VkApplicationInfo {
        sType: libappvulkan::bindings::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pApplicationName: c"Vulkan Example".as_ptr(),
        applicationVersion: libappvulkan::VK_MAKE_API_VERSION(0, 0, 1, 0),
        pEngineName: c"No Engine".as_ptr(),
        engineVersion: libappvulkan::VK_MAKE_API_VERSION(0, 1, 0, 0),
        apiVersion: libappvulkan::VK_API_VERSION_1_0,
        pNext: std::ptr::null(),
    };
    let create_info = libappvulkan::bindings::VkInstanceCreateInfo {
        sType: libappvulkan::bindings::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pApplicationInfo: &app_info,
        pNext: std::ptr::null(),
        flags: 0,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: std::ptr::null(),
    };

    let instance = libappvulkan::VkInstance::create_instance(&create_info)?;

    // create a dummy instance used to query the available extensions
    // let vk_instance = loader.vk_create_instance(&create_info,None)?;
    let layers = instance.vk_enumerate_instance_layer_properties()?;
    let extensions = instance.vk_enumerate_instance_extension_properties(None)?;

    // destroy the dummy instance
    instance.destroy()?;

    // println!("{extensions}");
    dbg!(layers, extensions);

    Ok(())
}
