
pub mod bindings {

    
    #[cfg(unix)]
    #[link(name = "dl", kind = "dylib")]
    extern "C" {
        #[link_name = "dlopen"]
        pub fn _dlopen(name: *const std::ffi::c_char, flags: std::ffi::c_int) -> *mut std::ffi::c_void;
        pub fn dlsym(
            handle: *mut std::ffi::c_void,
            symbol: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_void;
        pub fn dlclose(handle: *mut std::ffi::c_void) -> std::ffi::c_int;
        pub fn dlerror() -> *const std::ffi::c_char;
    }
    
    
    
    
    
    #[cfg(all(not(unix),target_os="windows"))]
    #[link(name="kernel32",kind="dylib")]
    unsafe extern "system" {
        #[link_name = "LoadLibraryA"]
        pub unsafe extern fn _dlopen(name: *const std::ffi::c_char) -> *mut std::ffi::c_void;
        
        #[link_name = "GetProcAddress"]
        pub unsafe extern fn dlsym(handle: *mut std::ffi::c_void,name: *const std::ffi::c_char)-> *mut std::ffi::c_void;
        
        pub unsafe fn dlerror() -> *const std::ffi::c_char {
            std::ptr::null()
        }
    }
    pub unsafe fn dlopen(name: *const std::ffi::c_char,flags: std::ffi::c_int) -> *mut std::ffi::c_void {
        #[cfg(target_os="windows")]
        {
            _dlopen(name)
        }
        #[cfg(unix)] {
            _dlopen(name, flags)
        }
    }
}
    
    pub unsafe fn load_gl() -> *mut std::ffi::c_void {
    let paths = [c"C:\\Windows\\System32\\opengl32.dll",c"/usr/lib/x86_64-linux-gnu/libGL.so",c"/usr/lib/libGL.so",c"/usr/lib/x86_64-linux-gnu/libGL.so"];
    let mut ptr: *mut std::ffi::c_void = std::ptr::null_mut(); 
        for path in paths {
            ptr = bindings::dlopen(path.as_ptr(), 0x2);
            if !ptr.is_null() {
                break;
            }
        }
    
    return ptr;
}







pub fn load_gl_function(name: &str) {}

pub fn main() {
    let gl = unsafe {load_gl()};

    dbg!(gl);
}
