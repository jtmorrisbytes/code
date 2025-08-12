impl Drop for VkLogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
impl Drop for VkPhysicalDevice {
    fn drop(&mut self) {}
}
impl Drop for VkSurface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}
impl Drop for self::VKInstance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) }
    }
}
impl QueueFamilyIndicies {
    fn is_complete(&self, need_present: bool) -> bool {
        self.graphics_family != u32::MAX && (!need_present || self.present_family.is_some())
    }
    fn find_queue_families(
        vk_instance: &VKInstance,
        device: vk::PhysicalDevice,
        surface: Option<&VkSurface>,
    ) -> Self {
        let families = vk_instance.get_physical_device_queue_family_properties(device);

        let mut graphics_family = None;
        let mut present_family = None;

        for (index, family) in families.iter().enumerate() {
            if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                graphics_family = Some(index as u32);
            }
            if let Some(surface) = surface {
                let supports_present =
                    surface.get_physical_device_surface_support(device, index as u32);
                if supports_present {
                    present_family = Some(index as u32)
                }
            }
        }
        Self {
            graphics_family: graphics_family
                .expect("Expected to find a graphics family queue index but was none"),
            present_family,
        }
    }
}



impl VkLogicalDevice {
    fn new(device: ash::Device) -> Self {
        Self { device }
    }
}

impl VkPhysicalDevice {

    
    pub fn pick_physical_device(vk_instance: &VKInstance, surface: Option<&VkSurface>) -> Self {
        let physical_devices = vk_instance.enumerate_physical_devices().expect(
            "Failed to enumerate physical devices for VkPhysicalDevice::pick_physical_devices",
        );
        let physical_devices: Vec<_> = physical_devices
            .clone()
            .into_iter()
            .filter_map(|physical_device| {
                // compile_error!("finish picking physical devices")
                let properties = unsafe {vk_instance.instance.get_physical_device_properties(physical_device)};

                let indices =
                    QueueFamilyIndicies::find_queue_families(vk_instance, physical_device, surface);
                if indices.is_complete(surface.is_some()) {
                    Some((physical_device, indices))
                } else {
                    None
                }
            })
            .collect();

        if physical_devices.len() == 0 {
            panic!("Expected support for physical device but no physical devices are available")
        }
        let (physical_device, qf) = physical_devices[0];
        Self {
            physical_device,
            queue_family_indicies: qf,
        }
    }
}
impl VkSurface {
    pub fn get_physical_device_surface_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        unsafe {
            self.surface_loader.get_physical_device_surface_support(
                physical_device,
                queue_family_index,
                self.surface,
            )
        }
        .unwrap_or(false)
    }
    // handles the platform specific surface creation code

    pub fn create_from_handles(
        entry: &Entry,
        instance: &ash::Instance,
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle,
    ) -> Result<VkSurface, Box<dyn std::error::Error>> {
        let surface_loader = ash::khr::surface::Instance::new(entry, instance);

        // handle all possible combinations of window handle and display handle combination
        match (window_handle, display_handle) {
            (RawWindowHandle::Win32(win32_window_handle), RawDisplayHandle::Windows(_)) => {
                let win32_instance_loader = ash::khr::win32_surface::Instance::new(entry, instance);
                let surface_create_info = vk::Win32SurfaceCreateInfoKHR::default()
                    .hinstance(win32_window_handle.hinstance.unwrap().get())
                    .hwnd(win32_window_handle.hwnd.get());
                let surface = unsafe {
                    win32_instance_loader.create_win32_surface(&surface_create_info, None)?
                };
                Ok(Self {
                    surface,
                    surface_loader,
                })
            }
            unimpl => todo!("{unimpl:?}"),
        }
    }
    pub fn create_headless() {}
}

impl VulkanBackend {
    pub fn create_graphics_queue(instance: &VKInstance, physical_device: &VkPhysicalDevice) {}
    pub fn instance_create_surface(
        &mut self,
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle,
    ) -> Result<VkSurface, Box<dyn std::error::Error>>
where {
        self.instance.create_surface(window_handle, display_handle)
    }
    pub fn instance_destroy_surface(
        vk_instance: &mut VKInstance,
    ) -> Result<(), Box<dyn std::error::Error>> {
        vk_instance.destroy_surface();
        Ok(())
    }
}
impl self::VKInstance {
    pub fn get_device_queue(
        vk_logical_device: &VkLogicalDevice,
        vk_physical_device: &VkPhysicalDevice,
    ) {
        let queue = unsafe {
            vk_logical_device.device.get_device_queue(
                vk_physical_device.queue_family_indicies.graphics_family,
                0,
            )
        };
    }
    pub fn create_logical_device(
        instance: &VKInstance,
        physical_device: &VkPhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
    ) -> VkLogicalDevice {
        let logical_device =
            unsafe { instance.instance.create_device(physical_device.physical_device, create_info, None) }
                .unwrap();
        let logical_device = VkLogicalDevice::new(logical_device);
        logical_device
    }
    pub fn get_physical_device_queue_family_properties(
        instance: &VKInstance,
        device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties> {
        unsafe { instance.instance.get_physical_device_queue_family_properties(device) }
    }
    pub fn enumerate_physical_devices(&self) -> Result<Vec<vk::PhysicalDevice>, vk::Result> {
        unsafe { self.instance.enumerate_physical_devices() }
    }
    pub fn get_physical_device_properties(
        instance: &VKInstance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        unsafe { instance.instance.get_physical_device_properties(physical_device) }
    }
    pub fn create(
        render_target: RenderTarget,
        app_name: &str,
        engine_name: &str,
        layers: &[*const i8],
        extensions: &[*const i8],
    ) -> ash::prelude::VkResult<Self> {
        let entry = unsafe { Entry::load().unwrap() };

        let app_name_c = CString::new(app_name).unwrap();
        let engine_name_c = CString::new(engine_name).unwrap();

        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name_c)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name_c)
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::API_VERSION_1_3);

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(extensions)
            .enabled_layer_names(&layers);

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok(Self { entry, instance })
    }
    pub fn create_surface(
        &self,
        raw_window_handle: RawWindowHandle,
        raw_display_handle: RawDisplayHandle,
    ) -> Result<VkSurface, Box<dyn std::error::Error>> {
        let surface = VkSurface::create_from_handles(
            &self.entry,
            &self.instance,
            raw_window_handle,
            raw_display_handle,
        )?;
        
        Ok(surface)
    }
    
}
#[derive(Clone, Copy)]
pub struct QueueFamilyIndicies {
    graphics_family: u32,
    present_family: Option<u32>,
}

pub struct VKInstance {
    entry: ash::Entry,
    instance: ash::Instance,
}

/// a wrapper struct for ash::Device that takes ownership of the 'Device' and implements drop for RAII cleanup.
pub struct VkLogicalDevice {
    device: ash::Device,
}

pub struct VkPhysicalDevice {
    physical_device: vk::PhysicalDevice,
    queue_family_indicies: QueueFamilyIndicies,
}

pub struct VkSurface {
    surface_loader: ash::khr::surface::Instance,
    surface: ash::vk::SurfaceKHR,
}

/// A Vulkan rendering pipeline that allows for optional headless rendering
pub struct VulkanBackend {
    entry: Entry,
    logical_device: VkLogicalDevice,
    physical_device: VkPhysicalDevice,
    graphics_queue: (),
    surface: Option<VkSurface>,
    instance: self::VKInstance,
    // surface_loader:Option<ash::ext::surf>
}

impl GraphicsBackend for VulkanBackend {
    fn init(render_target: RenderTarget) -> Result<Self, anyhow::Error> {
        // loads the vulkan library
        let entry = unsafe { Entry::load() }?;

        let mut extensions = vec![ash::khr::surface::NAME.as_ptr()];
        #[cfg(target_os = "windows")]
        {
            extensions.push(ash::khr::win32_surface::NAME.as_ptr())
        }
        #[cfg(unix)]
        {
            extensions.push(ash::khr::xlib_surface::NAME.as_ptr())
        }
        let layers = vec![];

        let instance = VKInstance::create(
            entry,
            "Vulkan App",
            "Jordan's basic vulkan engine",
            layers.as_slice(),
            extensions.as_slice(),
        )?;
        let surface = match render_target {
            RenderTarget::Windowed(raw_window_handle, raw_display_handle) => Some(
                VkSurface::create_from_handles(entry, instance, window_handle, display_handle)?,
            ),
            _ => None,
        };
        let physical_device = Self::pick_physical_device(vk_instance);

        let mut graphics_queue_create_info = vk::DeviceQueueCreateInfo::default().queue_family_index(physical_device.queue_family_indicies.graphics_family);
        graphics_queue_create_info.queue_count = 1;
        graphics_queue_create_info.queue_priorities(&[1.0]);

        let graphics_queue = ();

        // create the logical device
        let mut device_create_info =
            vk::DeviceCreateInfo::default().queue_create_infos([&graphics_queue_create_info]);
        
        instance.create_virtual_device()
        Ok(Self {
            instance,
            logical_device,
            physical_device,
            surface: None,
            graphics_queue,
            // presentation_queue,
            entry,
        })
    }
    fn render_frame(&mut self, scene: ()) {}
    fn resize(&mut self, width: u32, height: u32) {}
    fn resume(&mut self, options: &crate::engine::backend::BackendOptions) {}
    fn shutdown(&mut self) {}
    fn suspend(&mut self) {}
}

use ash::prelude::*;
use ash::{Entry, EntryFnV1_0, InstanceFnV1_0, vk};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use std::ffi::CString;

use crate::engine::backend::{GraphicsBackend, RenderTarget};
