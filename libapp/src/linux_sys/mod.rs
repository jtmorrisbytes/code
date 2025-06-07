// LINUX

#[allow(non_snake_case)]
#[derive(PartialEq,Eq,Debug)]
pub enum Architecture {
    x86_64,
    i386,
}
#[derive(PartialEq,Eq,Debug)]
pub struct LibraryInfo {
    pub name:String,
    pub source:String,
    pub arch:Architecture,
    pub libpath:std::path::PathBuf
}

pub struct Library {
    ptr: *const std::ffi::c_void
}
impl Library {
    pub(crate) fn new(ptr: *const std::ffi::c_void) -> Library {
        Self{ptr}
    }
}

pub struct Symbol {
    ptr: *const std::ffi::c_void
}
impl Symbol {
    pub (crate) fn new(ptr: *const std::ffi::c_void) -> Symbol {
        Self{ptr}
    }
}

impl Drop for Library {
    fn drop(&mut self){
        if let Err(err) = crate::linux_sys::rust_dlcose(self.ptr) {
            eprintln!("Warning: dlclose failed: {err}")
        }
    }
}



// the dlopen interface from linux
#[cfg(target_os="linux")]
    unsafe extern "C" {
        pub unsafe fn dlsym(handle: *const std::ffi::c_void,filename: *const std::ffi::c_char) -> *const std::ffi::c_void;
        pub unsafe fn dlopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int ) -> *const std::ffi::c_void;
        pub unsafe fn dlclose(handle: *const std::ffi::c_void) -> std::ffi::c_int;
        pub unsafe fn dlerror() -> *const std::ffi::c_char;
    }
pub fn rust_dlcose(ptr: *const std::ffi::c_void) -> Result<(),String> {
    let status = unsafe{ dlclose(ptr)};
    if status == 0 {
        return Ok(())
    }
    else {
        rust_dlerror()
    }

}
pub fn rust_dlerror() -> Result<(),String> {
    let ptr = unsafe {dlerror()};
    if ptr.is_null() {
        return Ok(());
    }
    let cstr = unsafe {std::ffi::CStr::from_ptr(ptr)};
    Err(String::from_utf8_lossy(cstr.to_bytes()).to_string())
}
    pub(crate) unsafe fn rust_dlsym<PtrType>(library: &Library,name: &str) -> Result<PtrType,String> {
        let _ = rust_dlerror().ok();
        let name = std::ffi::CString::new(name).map_err(|e| format!("couldnt convert the name into a cstring: {e}"))?;
        let ptr = unsafe {dlsym(library.ptr,name.as_ptr())};
        if ptr.is_null() {
            let _ = rust_dlerror()?;
        }
        Ok(std::mem::transmute_copy::<*const std::ffi::c_void, PtrType>(&ptr))
    }

pub fn rust_dlopen(){}