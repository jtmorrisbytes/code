// WINDOWS
#[repr(transparent)]
struct HMODULE(std::ffi::c_void);
#[repr(transparent)]
struct LPCSTR(std::ffi::c_void);

// any functions that require kernel32
#[link(name="Kernel32")]
unsafe extern "system" {
    fn LoadLibraryA(LPCSTR) -> HMODULE;
}
#[test]
fn test_loadLibraryA() {
    panic!("this works")
}

pub fn rust_dlopen(){}
pub fn rust_dlsym(){}
pub fn rust_dlerror(){}
pub fn rust_dlclose(){}
