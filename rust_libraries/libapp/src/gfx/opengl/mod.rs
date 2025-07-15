pub mod bindings;
#[cfg(unix)]
pub mod glx;
pub unsafe fn load_shared_object(path: &str) -> *mut std::ffi::c_void {
    let cstring_path =
        std::ffi::CString::new(path).expect("Valid rust string during cstring conversion");
    #[cfg(unix)]
    {
        return libc::dlopen(cstring_path.as_ptr(), 0x2);
    }
    #[cfg(target_os = "windows")]
    {
        let bytes = cstring_path.as_bytes_with_nul();
        let bytes_ptr = bytes.as_ptr();
        let ptr = windows::core::PCSTR::from_raw(bytes_ptr);
        let handle = windows::Win32::System::LibraryLoader::LoadLibraryA(ptr).unwrap();
        return handle.0;
    }
}
pub unsafe fn load_symbol(handle: *mut std::ffi::c_void, symbol: &str) -> *const std::ffi::c_void {
    let cstring_symbol_name =
        std::ffi::CString::new(symbol).expect("Valid rust string during cstring conversion");

    #[cfg(unix)]
    {
        return libc::dlsym(handle, cstring_symbol_name.as_ptr());
    }
    #[cfg(target_os = "windows")]
    {
        let module_handle = windows::Win32::Foundation::HMODULE(handle);

        // symbol name
        let bytes = cstring_symbol_name.as_bytes_with_nul();
        let bytes_ptr = bytes.as_ptr();
        let ptr = windows::core::PCSTR::from_raw(bytes_ptr);

        let symbol_handle =
            windows::Win32::System::LibraryLoader::GetProcAddress(module_handle, ptr);
        std::mem::transmute(symbol_handle)
    }
}
// WARNING: this function may invalidate all module function pointers and may cause UB if the pointer is null or is not a valid library handle
pub unsafe fn unload_shared_object(handle: *mut std::ffi::c_void) -> () {
    #[cfg(unix)]
    {
        let _result = libc::dlclose(handle);
    }
    #[cfg(windows)]
    {
        let module_handle = windows::Win32::Foundation::HMODULE(handle);
        windows::Win32::Foundation::FreeLibrary(module_handle);
    }
}
#[derive(Debug)]
pub struct Library {
    module: *mut std::ffi::c_void,
}
impl Library {
    pub fn new(path: &str) -> Self {
        Self {
            module: unsafe { load_shared_object(path) },
        }
    }
    pub fn load_symbol(&self, name: &str) -> *const std::ffi::c_void {
        unsafe { load_symbol(self.module, name) }
    }
}
impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            unload_shared_object(self.module);
        }
    }
}
#[cfg(target_os = "windows")]
pub struct WGLContext;

/// get a string from a glubyte ptr. may panic or cause UB if gl implementation does not nul terminate string!
pub unsafe fn const_glubyte_ptr_to_string(ptr: *const u8) -> String {
    // const MAX_LEN: usize = 128;
    if ptr.is_null() {
        return String::new();
    }
    if !ptr.is_aligned() {
        return String::new();
    }
    let mut buffer = vec![];
    let mut offset = 0;
    // attempt to read a u8, one byte at a time, by advancing the pointer by size_of u8, until 0
    unsafe {
        loop{
            let char_ptr = ptr.offset((std::mem::size_of::<u8>() * offset) as isize);
            if char_ptr.is_null(){
                break;
            }
            if !char_ptr.is_aligned() {
                break;
            }
            let char = *char_ptr;
            buffer.push(char);
            if char == 0 {
                break;
            }
            offset = offset + 1;
        }
    }
    std::ffi::CStr::from_bytes_until_nul(&buffer).map(|cstr|cstr.to_string_lossy().to_string()).unwrap_or(String::new())
}



/*
pub struct AttribsListBuilder{
r#use_gl: Option<bool>,

} */
