use std::ffi::CString;

use ash::prelude::*;
use ash::{Entry, EntryFnV1_0, InstanceFnV1_0, vk};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

pub struct VulkanSurface {
    surface_loader: ash::khr::surface::Instance,
    surface: ash::vk::SurfaceKHR,
}
impl VulkanSurface {
    // handles the platform specific surface creation code

    pub fn create_from_handles(
        entry: &Entry,
        instance: &ash::Instance,
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle,
    ) -> Result<VulkanSurface, Box<dyn std::error::Error>> {
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
impl Drop for VulkanSurface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}

pub struct VulkanInstance {
    entry: ash::Entry,
    surface: Option<VulkanSurface>,
    instance: ash::Instance,
}
impl Drop for self::VulkanInstance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) }
    }
}
impl self::VulkanInstance {
    pub fn create(
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

        Ok(Self {
            entry,
            instance,
            surface: None,
        })
    }
    pub fn create_surface(
        &mut self,
        raw_window_handle: RawWindowHandle,
        raw_display_handle: RawDisplayHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let surface = VulkanSurface::create_from_handles(
            &self.entry,
            &self.instance,
            raw_window_handle,
            raw_display_handle,
        )?;
        self.surface.replace(surface);
        Ok(())
    }
    pub fn destroy_surface(&mut self) {
        self.surface = None;
    }
    pub fn recreate_surface<Window>(&mut self, w: Window) -> Result<(), Box<dyn std::error::Error>>
    where
        Window: HasDisplayHandle + HasWindowHandle,
    {
        self.destroy_surface();

        self.create_surface(
            w.window_handle().unwrap().as_raw(),
            w.display_handle().unwrap().as_raw(),
        )
    }
}

/// A Vulkan rendering pipeline that allows for optional headless rendering
pub struct VulkanBackend {
    instance: Option<self::VulkanInstance>,
    entry: Entry,
    // surface_loader:Option<ash::ext::surf>
}

impl VulkanBackend {
    pub fn new() -> Self {
        let entry = unsafe { ash::Entry::load() }.unwrap();

        Self {
            instance: None,
            entry,
        }
    }
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // loads the vulkan library
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

        let instance = VulkanInstance::create(
            "Vulkan App",
            "Jordan's basic vulkan engine",
            layers.as_slice(),
            extensions.as_slice(),
        )?;
        self.instance.replace(instance);
        Ok(())
    }
    pub fn instance_create_surface(
        &mut self,
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
    {
        if self.instance.is_none() {
            return Err("Vulkan Instance has not been intialized yet".into());
        }
        let instance = self.instance.as_mut().unwrap();
        instance.create_surface(window_handle,display_handle)
    }
    pub fn instance_destroy_surface(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.instance.is_none() {
            return Err("Vulkan Instance has not been intialized before the call to VulkanBackend::instance_destroy_surfce(&mut self)".into());
        }
        let instance = self.instance.as_mut().unwrap();
        instance.destroy_surface();
        Ok(())
    }
}

impl Drop for VulkanBackend {
    fn drop(&mut self) {}
}
