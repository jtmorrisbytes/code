use std::ffi::CString;

// pub mod bindings;
pub mod extern_function_definitions;
pub const RLTD_NOW: std::ffi::c_int = 0x0002;
pub const LIBRARY_NAME_CSTR: &std::ffi::CStr = c"X11";

pub mod sys;



unsafe extern "C"{
    #[link_name = "dlopen"]
    pub unsafe fn c_dlopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int) -> *mut std::ffi::c_void;
    #[link_name = "dlsym"]
    pub unsafe fn c_dlsym(handle: *mut std::ffi::c_void,symbol_name: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    #[link_name="dlclose"]
    pub unsafe fn c_dlclose(handle: *mut std::ffi::c_void);
}

// #[derive(Debug,thiserror::Error)]
// #[error("Failed to open the X11 Shared object")]
// enum OpenX11SharedObjectError {

// }


pub struct Display {
    inner: *mut bindings::_XDisplay
}

impl Display {
    pub fn try_open(path:Option<&str>) -> Result<Self,String> {
        let ptr = self::x_open_display(path)?;
        Ok(Self{inner:ptr})
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        x_close_display(self.inner);
    }
}

macro_rules! call_extern_fn {
    ($name:ident, $($args:expr),*) => {
        #[cfg(test)]
        {
            let symbol_name = std::ffi::Cstring::new(stringify!($name)).unwrap();
            let ptr = c_dlopen(c"X11".as_ptr());
            if ptr.is_null() {
                panic!("Cannot call_extern_fn in package libxorg because dlopen cannot find libX11.so and the package author cannot figure out how to tell rust to link dynmically");
            }
            if !ptr.is_aligned() {
                panic!("Cannot call_extern_fn!() because the module pointer is not aligned");
            }
            
        }
        #[cfg(not(test))] {
            $name($($args),*)
        }
        
    };
}



#[test]
pub fn test_display_try_open() {
    let disp =  Display::try_open(None).unwrap();
    let disp = Display::try_open(Some(":1")).unwrap();
}


fn str_to_cstring(s: &str) -> std::ffi::CString {
    std::ffi::CString::new(s).expect("Valid rust string. NO NULL BYTES")
}
fn map_str_to_cstring(o_s:Option<&str>) -> Option<std::ffi::CString> {
    o_s.map(str_to_cstring)
}
fn map_cstring_or_else_nullpointer(o_cstr: Option<std::ffi::CString>) -> *const std::ffi::c_char {
    o_cstr.map(|s|s.as_ptr()).unwrap_or(std::ptr::null())
}
fn map_option_t_to_const_pointer<T>(o:Option<&T>) -> *const T {
    o.map(|t|std::ptr::from_ref(t)).unwrap_or(std::ptr::null())
}
fn map_option_mut_t_to_mut_pointer<T>(o:Option<&mut T>) -> *const T {
    o.map(|t | std::ptr::from_ref(t)).unwrap_or(std::ptr::null_mut())
}
fn check_mut_pointer_or_return_error<T>(ptr:*mut T) -> Result<(),String> {
    if ptr.is_null() {
        return Err(format!("*mut {}: failed null pointer check",std::any::type_name::<T>()))
    }
    else if !ptr.is_aligned() {
        return Err(format!("*mut {}: pointer is not aligned!",std::any::type_name::<T>()));
    }
    else if unsafe {ptr.as_ref()}.is_none() {
        return Err(format!("*mut {}: pointer is not valid",std::any::type_name::<T>()))
    }
    else {
        return Ok(())
    }

}

pub fn x_open_display(path:Option<&str>) -> Result<*mut bindings::_XDisplay,String> {
    let cstr = map_str_to_cstring(path);
    let s_ptr = map_cstring_or_else_nullpointer(cstr);

    let ptr = unsafe {bindings::XOpenDisplay(s_ptr)};
    let _ = check_mut_pointer_or_return_error(ptr)?;

    Ok(ptr)

}

pub fn x_close_display(dis: *mut bindings::Display) {

    todo!()
}



// pub fn open_x11_shared_object() -> Result<>