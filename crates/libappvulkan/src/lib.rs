/*
** Copyright 2015-2024 The Khronos Group Inc.
**
** SPDX-License-Identifier: Apache-2.0
*/
// code ported from vulkan headers in libvulkan-dev

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod win32;
#[allow(non_snake_case)]
pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29_u32) | (major << 22_u32) | (minor << 12_u32) | patch
}
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0, 1, 0, 0);

pub mod bindings;
pub mod custom_impls;
pub struct VkInstance {
    ptr: bindings::VkInstance,
    loader: Loader,
}
impl VkInstance {
    pub fn as_ptr(&self) -> bindings::VkInstance {
        self.ptr
    }
    pub fn destroy(self) -> Result<(), String> {
        self.loader
            .vk_destroy_instance(self.ptr, std::ptr::null())?;
        Ok(())
    }
    pub fn create_instance(create_info: &bindings::VkInstanceCreateInfo) -> Result<Self, String> {
        let loader = Loader::try_new()?;
        let ptr = loader.vk_create_instance(create_info, None)?;

        Ok(Self { ptr, loader })
    }
    pub fn vk_enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&str>,
    ) -> Result<Vec<bindings::VkExtensionProperties>, String> {
        self.loader
            .vk_enumerate_instance_extension_properties(Some(self.ptr), layer_name)
    }
    pub fn vk_enumerate_instance_layer_properties(
        &self,
    ) -> Result<Vec<bindings::VkLayerProperties>, String> {
        self.loader
            .vk_enumerate_instance_layer_properties(Some(self.ptr))
    }
    pub fn get_required_extensions_for_drawing_into_window(&self) -> Vec<&std::ffi::CStr> {
        cfg_if::cfg_if! {
           if #[cfg(target_os="windows")] {
            vec![self::win32::VK_KHR_WIN32_SURFACE_EXTENSION_NAME]
        }
        else if #[cfg(target_os="linux")] {
            let xdg_session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or(String::from("x11"));
            let s = xdg_session_type.as_str();
            match s {
                "x11" =>vec![self::linux::VK_KHR_XLIB_SURFACE_EXTENSION_NAME],
                "wayland" =>vec![self::linux::VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME],
                "unspecified" =>vec![],
                // text session
                "tty" =>vec![],
                // MIR display server?
                "mir" =>vec![],
                // a web (browser?) based session?
                "web" => vec![],
                unknown => {

                    println!("'{unknown}' is an unknown XDG_SESSION_TYPE. creating a window may not work");
                    vec![]
                }

            }

        }
        else if #[cfg(target_os="macos")] {
            vec![]
        }
        else if #[cfg(target_os="android")] {
            vec![]
        }
        else {
            println!("This is an unknown or unhandled target operating system: {}. returning an empty list",env!("CARGO_CFG_TARGET_OS"));
            vec![]
        }
        }
    }
}
// impl Drop for VkInstance {
//     /// dont forget to destroy all child objects before letting this drop
//     fn drop(&mut self) {
//         let _  =self.loader.vk_destroy_instance(self.ptr,std::ptr::null()).inspect_err(|e| eprintln!("{}",e));
//         // let _ = self.destroy();
//     }
// }

// attempt to load the vulkan api from the vulkan library at runtime.

// dynamically loads the vulkan loader function from platform apis

#[cfg(target_os = "linux")]
unsafe extern "C" {
    pub unsafe fn dlopen(
        filename: *const std::ffi::c_char,
        flags: std::ffi::c_int,
    ) -> *mut std::ffi::c_void;
    pub unsafe fn dlerror() -> *const std::ffi::c_char;
    pub unsafe fn dlclose(handle: *mut std::ffi::c_void);
    pub unsafe fn dlsym(
        handle: *mut std::ffi::c_void,
        symbol: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_void;
}
fn _dlopen(path: &str) -> Result<*mut std::ffi::c_void, String> {
    cfg_if::cfg_if! {
        if  #[cfg(target_os = "linux")] {
            let cstr = std::ffi::CString::new(path).unwrap();
            let ptr = unsafe { dlopen(cstr.as_ptr(), 0x00002) };
        if ptr.is_null() {
            let error = _dlerror().unwrap_or_else(|| format!("Failed to open shared object at {path}:  recieved null pointer from C function dlopen(). No error was available"));
            eprintln!("{error}");
            return Err(error);
        }
        Ok(ptr)

        }
        else if #[cfg(target_os="windows")] {
            unsafe {self::win32::_dlopen(path)}
        }
        else {
            compile_error!("function _dlopen is not implemented for {}!",env!("CARGO_CFG_TARGET_OS"))
        }
    }
}
#[cfg(target_os = "linux")]
fn _dlerror() -> Option<String> {
    let err = unsafe { dlerror() };
    if err.is_null() {
        return None;
    }
    let cstr = unsafe { std::ffi::CStr::from_ptr(err) };
    let str = cstr
        .to_str()
        .expect("Valid UTF-8 or ANSI sequence from C function dlerror()");
    let string = str.to_string();
    Some(string)
}

unsafe fn _dlsym(
    handle: *mut std::ffi::c_void,
    symbol_name: &str,
) -> Result<*mut std::ffi::c_void, String> {
    if handle.is_null() {
        return Err(format!("Receved null pointer from rust function while trying to look up a symbol: {symbol_name}"));
    }
    cfg_if::cfg_if! {
        if #[cfg(target_os="linux")] {

            let symbol_cstr = std::ffi::CString::new(symbol_name).map_err(|e| e.to_string())?;
            let p_symbol = unsafe { dlsym(handle, symbol_cstr.as_ptr()) };
            if p_symbol.is_null() {
                let error = _dlerror().unwrap_or_else(||{format!("Recieved null pointer from C function dlsym() while trying to look up a symbol {symbol_name}. No error message is available")});
                return Err(error);
            }
            Ok(p_symbol)
        }
        else if #[cfg(target_os="windows")] {
            // dll loading errors should be handled by windows::core::Error::from_win32
            self::win32::_dlsym(handle,symbol_name)
        }
        else {
            compile_error!("_dlsym is not implemented for target_os='{}'. implement id",env!("CARGO_CFG_TARGET_OS"))
        }
    }
}
pub struct Loader {
    shared_object: *mut std::ffi::c_void,
    pfn_vk_get_instance_proc_address: crate::bindings::PFN_vkGetInstanceProcAddr,
    // pfn_vk_create_instance: crate::bindings::PFN_vkCreateInstance
}
// impl Drop for Loader {
//     fn drop(&mut self) {
//         unsafe { dlclose(self.shared_object) };
//     }
// }
// macro_rules! transmute {
//     ($ptr:ident,$t:ty) => {
//         unsafe {
//             let __ptr = $ptr as *const ();
//             std::mem::transmute::<_, $t>(__ptr)
//         }
//     };
// }
unsafe fn _transmute<T>(ptr: *mut std::ffi::c_void) -> T {
    let _ptr = ptr as *const ();
    unsafe { std::mem::transmute_copy::<_, T>(&*_ptr) }
}
unsafe fn load_symbol<T>(handle: *mut std::ffi::c_void, symbol_name: &str) -> Result<T, String> {
    let ptr = unsafe { _dlsym(handle, symbol_name) }?;
    let t = unsafe { _transmute::<T>(ptr) };
    Ok(t)
}

impl Loader {
    pub unsafe fn load_symbol<T>(&self, symbol_name: &str) -> Result<T, String> {
        let ptr = unsafe { _dlsym(self.shared_object, symbol_name) }?;
        let t = unsafe { _transmute::<T>(ptr) };
        Ok(t)
    }
    pub fn try_new() -> Result<Self, String> {
        let path = {
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            {
                "/lib/x86_64-linux-gnu/libvulkan.so"
            }
            #[cfg(all(target_os = "linux", target_arch = "x86"))]
            {
                "/lib/i386-linux-gnu/libvulkan.so"
            }
            #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
            {
                "C:\\Windows\\System32\\vulkan-1.dll"
            }
            #[cfg(all(target_os = "windows", target_arch = "x86"))]
            {
                "C:\\Windows\\SysWOW64\\vulkan-1.dll"
            }
        };
        let shared_object: *mut std::ffi::c_void = self::_dlopen(path)?;
        if shared_object.is_null() {
            panic!("Shared object was null even after checking!");
        }
        // load all essential functions
        let pfn_vk_createinstance = unsafe { _dlsym(shared_object, "vkGetInstanceProcAddr")? };
        let pfn_vk_get_instance_proc_addr = transmute!(
            pfn_vk_createinstance,
            crate::bindings::PFN_vkGetInstanceProcAddr
        )
        .ok_or("Failed to get pointer to vkCreateInstance".to_string())?;
        Ok(Self {
            shared_object,
            pfn_vk_get_instance_proc_address: Some(pfn_vk_get_instance_proc_addr),
        })
    }
    pub fn vk_get_instance_proc_addr(
        &self,
        instance: bindings::VkInstance,
        name: &str,
    ) -> Result<Option<unsafe extern "C" fn()>, String> {
        if self.pfn_vk_get_instance_proc_address.is_none() {
            return Err("function vkGetInstanceProcedureAddr is not available, cannot call vkGetInstanceProcedureAddr from this interface!".to_string());
        }
        let p_fn = self.pfn_vk_get_instance_proc_address.unwrap();
        let cstr = std::ffi::CString::new(name).expect("Non null terminated rust strings!");
        let procaddr = unsafe { p_fn(instance, cstr.as_ptr()) };
        Ok(procaddr)
    }
    pub fn vk_create_instance(
        &self,
        vk_instance_create_info: &bindings::VkInstanceCreateInfo,
        allocation_callbacks: Option<crate::bindings::VkAllocationCallbacks>,
    ) -> Result<bindings::VkInstance, String> {
        // call this function with NULL, vkCreateInstance
        let fn_pointer =
            self.vk_get_instance_proc_addr(std::ptr::null_mut(), "vkCreateInstance")?;
        let function = unsafe {
            std::mem::transmute::<Option<unsafe extern "C" fn()>, bindings::PFN_vkCreateInstance>(
                fn_pointer,
            )
        };
        if function.is_none() {
            return Err("vkCreateInstance was null".to_string());
        }
        let function = unsafe { function.unwrap_unchecked() };
        let allocation_callbacks = allocation_callbacks
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut instance: crate::bindings::VkInstance = std::ptr::null_mut();
        let vk_result: crate::bindings::VkResult =
            unsafe { function(vk_instance_create_info, allocation_callbacks, &mut instance) };

        match vk_result {
                crate::bindings::VkResult_VK_SUCCESS => {
                    if instance.is_null() {
                        Err("Null Instance after checking vk_result!".to_string())
                    }
                    else {
                        Ok(instance)
                    }
                }
                crate::bindings::VkResult_VK_ERROR_OUT_OF_DEVICE_MEMORY => {
                    Err("vkCreateInstance: Out of DEVICE memory".to_string())
                }
                crate::bindings::VkResult_VK_ERROR_OUT_OF_HOST_MEMORY => {
                    Err("vkCreateInstance: Out of HOST memory".to_string())
                }
                crate::bindings::VkResult_VK_ERROR_INITIALIZATION_FAILED => {
                    Err("vkCreateInstance: Initialization failed".to_string())
                }
                crate::bindings::VkResult_VK_ERROR_LAYER_NOT_PRESENT => Err("vkCreateInstance: A requested validation layer was not loaded or is not available".to_string()),
                crate::bindings::VkResult_VK_ERROR_EXTENSION_NOT_PRESENT => Err("vkCreateInstance: A requested extension is not present or is not available".to_string()),
                crate::bindings::VkResult_VK_ERROR_INCOMPATIBLE_DRIVER => Err("vkCreateInstance: The driver that vulkan tried is not compatible".to_string()),
                _=> {
                    Err(format!("The function vkCreateInstance returned a VkResult value which is not understood by this application: {vk_result}"))
                }
            }
    }
    pub fn vk_destroy_instance(
        &self,
        vk_instance: bindings::VkInstance,
        allocation_callbacks: *const crate::bindings::VkAllocationCallbacks,
    ) -> Result<(), String> {
        let ptr = unsafe { _dlsym(self.shared_object, "vkDestroyInstance")? };

        assert!(!ptr.is_null());

        let pfn = ptr as *const ();
        let pfn: bindings::PFN_vkDestroyInstance = unsafe { std::mem::transmute(ptr) };

        // let pfn = self.vk_get_instance_proc_addr(vk_instance,"vkDestroyInstance")?;
        // let pfn: bindings::PFN_vkDestroyInstance = unsafe {std::mem::transmute(pfn)};
        let pfn = pfn.ok_or_else(|| "vkDestroyInstance is not available".to_string())?;
        assert!(!vk_instance.is_null());
        dbg!(vk_instance);
        unsafe { pfn(vk_instance, allocation_callbacks) };
        Ok(())
    }
    pub fn vk_enumerate_instance_extension_properties(
        &self,
        vk_instance: Option<bindings::VkInstance>,
        layer_name: Option<&str>,
    ) -> Result<Vec<bindings::VkExtensionProperties>, String> {
        let pfn = self.vk_get_instance_proc_addr(
            vk_instance.unwrap_or(std::ptr::null_mut()),
            "vkEnumerateInstanceExtensionProperties",
        )?;
        let pfn_vk_enumerate_instance_extension_properties = pfn
            .map(|pfn| unsafe {
                std::mem::transmute::<
                    _,
                    unsafe extern "C" fn(
                        *const ::std::os::raw::c_char,
                        *mut u32,
                        *mut bindings::VkExtensionProperties,
                    ) -> bindings::VkResult,
                >(pfn as *const ())
            })
            .unwrap();
        let mut count = 0;

        let layer_name = layer_name.map(|layer_name| {
            std::ffi::CString::new(layer_name).expect("NonNull terminated string!")
        });

        let layer_name_ptr = layer_name
            .map(|layer_name| layer_name.as_ptr())
            .unwrap_or(std::ptr::null());
        // let mut properties: bindings::VkExtensionProperties = unsafe { std::mem::zeroed() };

        let _vkresult: bindings::VkResult = unsafe {
            pfn_vk_enumerate_instance_extension_properties(
                layer_name_ptr,
                &mut count,
                std::ptr::null_mut(),
            )
        };
        if _vkresult != bindings::VkResult_VK_SUCCESS {
            eprintln!("{_vkresult}");
            return Err(format!(
                "Failed to get the number of insance extensions: {_vkresult}"
            ));
        }
        let mut properties: Vec<bindings::VkExtensionProperties> = Vec::new();
        for _ in 0..count {
            properties.push(unsafe { std::mem::zeroed() })
        }

        dbg!(count);
        let _vkresult = unsafe {
            pfn_vk_enumerate_instance_extension_properties(
                layer_name_ptr,
                &mut count,
                properties.as_mut_ptr(),
            )
        };
        if _vkresult != bindings::VkResult_VK_SUCCESS {
            eprintln!("{_vkresult}");
            return Err(format!(
                "Failed populate the list of instance extensions: {_vkresult}"
            ));
        }

        Ok(properties)
    }
    pub fn vk_enumerate_instance_layer_properties(
        &self,
        vk_instance: Option<bindings::VkInstance>,
    ) -> Result<Vec<bindings::VkLayerProperties>, String> {
        const FN_NAME: &str = "vkEnumerateInstanceLayerProperties";
        let pfn_vk_enumerate_instance_layer_properties = self.vk_get_instance_proc_addr(
            vk_instance.unwrap_or(std::ptr::null_mut()),
            "vkEnumerateInstanceLayerProperties",
        )?;
        let pfn_vk_enumerate_instance_layer_properties = unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                bindings::PFN_vkEnumerateInstanceLayerProperties,
            >(pfn_vk_enumerate_instance_layer_properties)
        };
        let mut layer_count = 0;

        if pfn_vk_enumerate_instance_layer_properties.is_none() {
            return Err(format!("vulkan function {FN_NAME} is not available: vkGetInstanceProcAddress returned null!"));
        }
        let pfn_vk_enumerate_instance_layer_properties =
            pfn_vk_enumerate_instance_layer_properties.unwrap();
        let vkresult = unsafe {
            pfn_vk_enumerate_instance_layer_properties(&mut layer_count, std::ptr::null_mut())
        };
        if vkresult != bindings::VkResult_VK_SUCCESS {
            return Err(format!(
                "Failed to get the number of insance layer properties: {vkresult}"
            ));
        }

        let mut properties: Vec<bindings::VkLayerProperties> = Vec::new();
        // initialize the array. this is the only thing that seems to work
        for _ in 0..layer_count {
            properties.push(unsafe { std::mem::zeroed() })
        }
        let vkresult = unsafe {
            pfn_vk_enumerate_instance_layer_properties(&mut layer_count, std::ptr::null_mut())
        };
        if vkresult != bindings::VkResult_VK_SUCCESS {
            return Err(format!(
                "Failed to get the number of insance layer properties: {vkresult}"
            ));
        }

        Ok(properties)
    }
}

#[test]
pub fn test_libappvulkan_loader() -> Result<(), Box<dyn std::error::Error>> {
    let loader = Loader::try_new()?;

    let vk_instance_create_info: crate::bindings::VkInstanceCreateInfo =
        unsafe { std::mem::zeroed() };
    let instance = loader.vk_create_instance(&vk_instance_create_info, None)?;

    Ok(())
}
#[test]
pub fn test_libappvulkan_loader_enumerate_extension_properties(
) -> Result<(), Box<dyn std::error::Error>> {
    let loader = Loader::try_new()?;
    let properties = loader.vk_enumerate_instance_extension_properties(None, None)?;
    Ok(())
}

#[test]
pub fn test_libappvulkan_loader_enumerate_layer_properties(
) -> Result<(), Box<dyn std::error::Error>> {
    let loader = Loader::try_new()?;
    let loader = Loader::try_new()?;
    let loader = Loader::try_new()?;

    let properties = loader.vk_enumerate_instance_layer_properties(None)?;
    println!("{properties:?}");
    Ok(())
}
