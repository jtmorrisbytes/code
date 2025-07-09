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



#[cfg(unix)]
pub mod glx {
    use std::i32;

    use winit::raw_window_handle::RawDisplayHandle;
    use winit::raw_window_handle::RawWindowHandle;
    use winit::raw_window_handle::XlibDisplayHandle;

    use crate::gfx::opengl::const_glubyte_ptr_to_string;

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

        pub fn get_gl_version(&self) -> () {
            match self {
                Self::V1_4 { library, context, glx, display, window } => {
                    let gl = bindings::gl::v1_0::Gl::load_with(|name|library.load_symbol(name));
                    let s = unsafe {gl.GetString(bindings::gl::v1_1::VERSION)};
                    let s = unsafe {const_glubyte_ptr_to_string(s)};
                    dbg!(s);
                }
                _=>{todo!()}
            }
        }
        pub fn draw(&mut self) {

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

    pub fn glx_get_version_from_xlib(library: &Library,
        dpy: *mut bindings::glx::types::Display,
    ) -> GLXVersion {
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        let glx: bindings::glx::v1_0::Glx =
            bindings::glx::v1_0::Glx::load_with(|name| library.load_symbol(name));

        unsafe {
            glx.QueryVersion(
                dpy,
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
        library: &Library,
        glx_version: &GLXVersion,
        display: *mut bindings::glx::types::Display,
        screen: bindings::glx::types::Screen,
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
        // let libgl = Library::new("/usr/lib/x86_64-linux-gnu/libGL.so");

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
                let glx = bindings::glx::v1_0::Glx::load_with(|name| library.load_symbol(name));
                unsafe { glx.ChooseVisual(display, screen, attribs_ptr) }
            }
            GLXVersion::V1_1 => {
                attribs.push(0);
                let glx = bindings::glx::v1_1::Glx::load_with(|name| library.load_symbol(name));
                unsafe { glx.ChooseVisual(display, screen, attribs_ptr) }
            }
            GLXVersion::V1_2 => {
                attribs.push(0);
                let glx = bindings::glx::v1_2::Glx::load_with(|name| library.load_symbol(name));
                unsafe { glx.ChooseVisual(display, screen, attribs_ptr) }
            }
            GLXVersion::V1_3 => {
                attribs.push(0);

                let glx = bindings::glx::v1_3::Glx::load_with(|name| library.load_symbol(name));
                // try choosefbconfig first
                unsafe { glx.ChooseVisual(display, screen, attribs_ptr) }
            }
            GLXVersion::V1_4 => {
                attribs.push(bindings::glx::v1_4::SAMPLES as i32);
                attribs.push(samples_count.try_into().unwrap_or(0));
                attribs.push(bindings::glx::v1_4::SAMPLE_BUFFERS as i32);
                attribs.push(sample_buffers_count as i32);
                attribs.push(0);
                let glx = bindings::glx::v1_4::Glx::load_with(|name| library.load_symbol(name));
                unsafe { glx.ChooseVisual(display, screen, attribs_ptr) }
            }
        };
        if visualinfo.is_null() {
            return Err(());
        }
        Ok(visualinfo)
    }
    // get the supported opengl version using platform specific apis from a winit window handle
    pub fn glx_create_context_from_winit_xlib_handle(display_handle: XlibDisplayHandle,window_handle: winit::raw_window_handle::XlibWindowHandle) -> Result<GLXContext,()> {
        let display = display_handle.display.unwrap().as_ptr().cast();
        let screen = display_handle.screen;
        let window = window_handle.window;
        let visual_id = window_handle.visual_id;
        glx_create_context(display,screen,window,visual_id)
    }
    
    
    pub fn glx_create_context(
        // glx_version: GLXVersion,
        display: *mut bindings::glx::types::Display,
        screen: bindings::glx::types::Screen,
        window: bindings::glx::types::Window,
        visualid: bindings::glx::types::VisualID
        // visualinfo: *mut bindings::glx::types::XVisualInfo,
    ) -> Result<GLXContext, ()> {
        let libgl = Library::new("/usr/lib/x86_64-linux-gnu/libGL.so");
        let glx_version = glx_get_version_from_xlib(&libgl,display);
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
                            &libgl,
                            &glx_version,
                            display,
                            screen,
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
                                display,
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
                            display: display,
                            window:window
                        })
                    }
                }
        }
}

pub trait GLContext {}

/*
pub struct AttribsListBuilder{
r#use_gl: Option<bool>,

} */
