pub mod bindings;

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

#[cfg(unix)]
pub mod glx {
    use std::i32;

    use winit::raw_window_handle::RawDisplayHandle;
    use winit::raw_window_handle::RawWindowHandle;

    use super::bindings;
    use super::Library;
    #[derive(Debug)]
    pub enum GLXContext {
        V1_0 {
            library: Library,
            context: *const std::ffi::c_void,
            glx: bindings::glx::v1_0::Glx,
            display: *mut bindings::glx::types::Display,
            window: bindings::glx::types::Window
        },
        V1_1 {
            library: Library,
            context: *const std::ffi::c_void,
            glx: bindings::glx::v1_1::Glx,
            display: *mut bindings::glx::types::Display,
            window: bindings::glx::types::Window
        },
        V1_2 {
            library: Library,
            context: *const std::ffi::c_void,
            glx: bindings::glx::v1_2::Glx,
            display: *mut bindings::glx::types::Display,
            window: bindings::glx::types::Window
        },
        V1_3 {
            library: Library,
            context: *const std::ffi::c_void,
            glx: bindings::glx::v1_3::Glx,
            display: *mut bindings::glx::types::Display,
            window: bindings::glx::types::Window
        },
        V1_4 {
            library: Library,
            context: *const std::ffi::c_void,
            glx: bindings::glx::v1_4::Glx,
            display: *mut bindings::glx::types::Display,
            window: bindings::glx::types::Window
        },
    }
    impl GLXContext {
        pub fn is_direct(&self) -> bool {
            match self {
                Self::V1_0 {
                    library: _,
                    context,
                    glx,
                    display,
                    window:_
                } => unsafe { glx.IsDirect(*display, *context) != 0 },
                Self::V1_1 {
                    library: _,
                    context,
                    glx,
                    display,
                    window:_,
                } => unsafe { glx.IsDirect(*display, *context) != 0 },
                Self::V1_2 {
                    library: _,
                    context,
                    glx,
                    display,
                    window:_
                } => unsafe { glx.IsDirect(*display, *context) != 0 },
                Self::V1_3 {
                    library: _,
                    context,
                    glx,
                    display,
                    window:_,
                } => unsafe { glx.IsDirect(*display, *context) != 0 },
                Self::V1_4 {
                    library: _,
                    context,
                    glx,
                    display,
                    window:_
                } => unsafe { glx.IsDirect(*display, *context) != 0 },
            }
        }
        pub fn make_current(&self) -> bool {
            match self {
                Self::V1_0 {
                    library: _,
                    context,
                    glx,
                    display,
                    window
                } => {
                    unsafe { glx.MakeCurrent(*display, *window, *context) != 0 }
                }
                Self::V1_1 {
                    library: _,
                    context,
                    glx,
                    display,
                    window
                } => unsafe { glx.MakeCurrent(*display, *window, *context) != 0 },
                Self::V1_2 {
                    library: _,
                    context,
                    glx,
                    display,
                    window,
                } => unsafe { glx.MakeCurrent(*display, *window,*context) != 0 },
                Self::V1_3 {
                    library: _,
                    context,
                    glx,
                    display,
                    window
                } => unsafe { glx.MakeCurrent(*display, *window, *context) != 0 },
                Self::V1_4 {
                    library: _,
                    context,
                    glx,
                    display,
                    window,
                } => unsafe { glx.MakeCurrent(*display,*window, *context) != 0 },
            }
        }
    }
    // **** GLX UTILS ******
    #[derive(Debug, PartialEq, Eq)]
    pub enum GLXVersion {
        V1_0,
        V1_1,
        V1_2,
        V1_3,
        V1_4,
    }
    // loads the appropriate GL library for X11, attempts to use glx v1.0 to get the supported version, then returns it

    pub fn glx_get_version_from_xlib(
        handle: winit::raw_window_handle::XlibDisplayHandle,
    ) -> GLXVersion {
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        // need the ability to search these
        let library = Library::new("/usr/lib/x86_64-linux-gnu/libGL.so");
        let glx: bindings::glx::v1_0::Glx =
            bindings::glx::v1_0::Glx::load_with(|name| library.load_symbol(name));

        unsafe {
            glx.QueryVersion(
                handle.display.unwrap().as_ptr().cast(),
                &mut major,
                &mut minor,
            )
        };
        match (major, minor) {
            (1, 0) => GLXVersion::V1_0,
            (1, 1) => GLXVersion::V1_1,
            (1, 2) => GLXVersion::V1_2,
            (1, 3) => GLXVersion::V1_3,
            (1, 4) => GLXVersion::V1_4,
            _ => panic!("Unsupported GLX Version {major}.{minor}"),
        }
    }

    pub struct GLXAttribs(Vec<std::ffi::c_int>);

    pub fn glx_choose_visual(
        glx_version: &GLXVersion,
        window_handle: winit::raw_window_handle::XlibDisplayHandle,
        use_rgba: bool,
        prefer_double_buffer: bool,
        depth_size: u32,
        stencil_size: u32,
        red_channel_size: u32,
        green_channel_size: u32,
        blue_channel_size: u32,
        sample_buffers_count: u32,
        samples_count: u32,
    ) -> Result<*mut bindings::glx::types::XVisualInfo, ()> {
        let libgl = Library::new("/usr/lib/x86_64-linux-gnu/libGL.so");
        let dpy = window_handle.display.unwrap().as_ptr().cast();
        let screen = window_handle.screen;

        let mut attribs: Vec<i32> = Vec::new();
        if use_rgba {
            attribs.push(bindings::glx::RGBA as i32);
        }
        if prefer_double_buffer {
            attribs.push(bindings::glx::DOUBLEBUFFER as i32)
        }
        // depth size
        attribs.push(bindings::glx::DEPTH_SIZE as i32);
        attribs.push(depth_size.try_into().unwrap_or(0));
        // stencil size
        attribs.push(bindings::glx::STENCIL_SIZE as i32);
        attribs.push(stencil_size.try_into().unwrap_or(0));

        attribs.push(bindings::glx::RED_SIZE as i32);
        attribs.push(red_channel_size.try_into().unwrap_or(0));

        attribs.push(bindings::glx::GREEN_SIZE as i32);
        attribs.push(green_channel_size.try_into().unwrap_or(0));

        attribs.push(bindings::glx::BLUE_SIZE as i32);
        attribs.push(blue_channel_size.try_into().unwrap_or(0));

        let attribs_ptr = attribs.as_mut_ptr();
        let visualinfo = match glx_version {
            GLXVersion::V1_0 => {
                attribs.push(0);
                let glx = bindings::glx::v1_0::Glx::load_with(|name| libgl.load_symbol(name));
                unsafe { glx.ChooseVisual(dpy, screen, attribs_ptr) }
            }
            GLXVersion::V1_1 => {
                attribs.push(0);
                let glx = bindings::glx::v1_1::Glx::load_with(|name| libgl.load_symbol(name));
                unsafe { glx.ChooseVisual(dpy, screen, attribs_ptr) }
            }
            GLXVersion::V1_2 => {
                attribs.push(0);
                let glx = bindings::glx::v1_2::Glx::load_with(|name| libgl.load_symbol(name));
                unsafe { glx.ChooseVisual(dpy, screen, attribs_ptr) }
            }
            GLXVersion::V1_3 => {
                attribs.push(0);

                let glx = bindings::glx::v1_3::Glx::load_with(|name| libgl.load_symbol(name));
                // try choosefbconfig first
                unsafe { glx.ChooseVisual(dpy, screen, attribs_ptr) }
            }
            GLXVersion::V1_4 => {
                attribs.push(bindings::glx::v1_4::SAMPLES as i32);
                attribs.push(samples_count.try_into().unwrap_or(0));
                attribs.push(bindings::glx::v1_4::SAMPLE_BUFFERS as i32);
                attribs.push(0);
                let glx = bindings::glx::v1_4::Glx::load_with(|name| libgl.load_symbol(name));
                unsafe { glx.ChooseVisual(dpy, screen, attribs_ptr) }
            }
        };
        if visualinfo.is_null() {
            return Err(());
        }
        Ok(visualinfo)
    }
    // get the supported opengl version using platform specific apis from a winit window handle
    pub trait WindowAndDisplayHandle:
        winit::raw_window_handle::HasDisplayHandle + winit::raw_window_handle::HasWindowHandle
    {
    }
    impl<T> WindowAndDisplayHandle for T where T: winit::raw_window_handle::HasDisplayHandle + winit::raw_window_handle::HasWindowHandle{}
    pub fn glx_create_context(
        // glx_version: GLXVersion,
        handle: Box<dyn WindowAndDisplayHandle>,
        // visualinfo: *mut bindings::glx::types::XVisualInfo,
    ) -> Result<GLXContext, ()> {
        let display_handle = handle.display_handle().unwrap().as_raw();
        let window_handle = handle.window_handle().unwrap().as_raw();
        match (window_handle, display_handle) {
            (RawWindowHandle::Xlib(window), RawDisplayHandle::Xlib(display_handle)) => {
                let libgl = Library::new("/usr/lib/x86_64-linux-gnu/libGL.so");
                let glx_version = glx_get_version_from_xlib(display_handle);
                let display_ptr = display_handle.display.unwrap().as_ptr().cast();

                match glx_version {
                    GLXVersion::V1_0 => {
                        todo!("GLX create context 1_0")
                    }
                    GLXVersion::V1_1 => {
                        todo!("GLX create context 1_1")
                    }
                    GLXVersion::V1_2 => {
                        todo!("GLX create context 1_2")
                    }
                    GLXVersion::V1_3 => {
                        todo!("GLX create context 1_3")
                    }
                    GLXVersion::V1_4 => {
                        let glx =
                            bindings::glx::v1_4::Glx::load_with(|name| libgl.load_symbol(name));
                        let visualinfo =glx_choose_visual(
                            &glx_version,
                            display_handle,
                            true,
                            true,
                            24,
                            8,
                            8,
                            8,
                            8,
                            0,
                            0,
                        )
                        .unwrap();
                        let ctx = unsafe {
                            glx.CreateContext(
                                display_ptr,
                                visualinfo,
                                std::ptr::null(),
                                1,
                            )
                        };
                        if ctx.is_null() {
                            return Err(());
                        }
                        if !ctx.is_aligned() {
                            return Err(());
                        }
                        Ok(GLXContext::V1_4 {
                            library: libgl,
                            context: ctx,
                            glx,
                            display: display_ptr,
                            window:window.window
                        })
                    }
                }
            }
            _ => todo!("window and display handles"),
        }
    }
}

pub trait GLContext {}

/*
pub struct AttribsListBuilder{
r#use_gl: Option<bool>,

} */
