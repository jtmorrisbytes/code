pub mod bindings {

    #[cfg(unix)]
    #[link(name = "dl", kind = "dylib")]
    extern "C" {
        #[link_name = "dlopen"]
        pub fn _dlopen(
            name: *const std::ffi::c_char,
            flags: std::ffi::c_int,
        ) -> *mut std::ffi::c_void;
        pub fn dlsym(
            handle: *mut std::ffi::c_void,
            symbol: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_void;
        pub fn dlclose(handle: *mut std::ffi::c_void) -> std::ffi::c_int;
        pub fn dlerror() -> *const std::ffi::c_char;
    }

    pub unsafe fn dlopen(name: &str, flags: std::ffi::c_int) -> *mut std::ffi::c_void {
        let name = std::ffi::CString::new(name).unwrap_or_default();
        #[cfg(target_os = "windows")]
        {
            let name_bytes = name.as_bytes_with_nul();
            let name_ptr = windows::core::PCSTR::from_raw(name_bytes.as_ptr());
            let ptr = windows::Win32::System::LibraryLoader::LoadLibraryA(name_ptr).unwrap();
            ptr.0
        }
        #[cfg(unix)]
        {
            _dlopen(name.as_ptr(), flags)
        }
    }
    #[cfg(unix)]
    pub mod unix {
        use super::super::Library;

        type XID = std::ffi::c_ulong;
        pub type Window = XID;
        pub type Colormap = XID;

        #[repr(C)]
        pub struct Display {}
        #[derive(Debug, Clone)]
        #[repr(C)]
        pub struct Screen {
            // unused
            ext_data: *const std::ffi::c_void,
            display: *mut Display,
            root: Window,
            width: std::ffi::c_int,
            height: std::ffi::c_int,
            mwidth: std::ffi::c_int,
            mheight: std::ffi::c_int,
            ndepths: std::ffi::c_int,
            depths: *mut std::ffi::c_void,
            root_depth: std::ffi::c_int,
            root_visual: *mut std::ffi::c_void,
            default_gc: *const std::ffi::c_void,
            colormap: Colormap,
            white_pixel: std::ffi::c_ulong,
            black_pixel: std::ffi::c_ulong,
        }

        pub struct XLibLibrary {
            library: super::super::Library,
        }
        impl XLibLibrary {
            pub fn try_load() -> Result<Self, ()> {
                for path in [
                    "/usr/lib/x86_64-linux-gnu/libX11.so",
                    "/usr/lib/i386-linux-gnu/libX11.so",
                    "/usr/lib/libX11.so",
                ] {
                    let l = Library::try_load(path);
                    if let Ok(l) = l {
                        return Ok(Self { library: l });
                    }
                }
                return Err(());
            }
            pub fn x_open_display(&self) -> *mut Display {
                let f = self.library.load_symbol("XOpenDisplay");
                assert!(!f.is_null());
                let p: unsafe extern "C" fn(*const std::ffi::c_char) -> *mut Display =
                    unsafe { std::mem::transmute(f) };
                let display = unsafe { p(std::ptr::null()) };
                if display.is_null() {
                    panic!("Null display");
                }
                display
            }
            pub fn x_default_screen_of_display(&self, display: *mut Display) -> *mut Screen {
                let ptr = self.library.load_symbol("XDefaultScreenOfDisplay");
                assert!(!ptr.is_null());
                let func: unsafe extern "C" fn(*mut Display) -> *mut Screen =
                    unsafe { std::mem::transmute(ptr) };
                unsafe { func(display) }
            }
        }
    }
}

pub struct Library {
    module: *mut std::ffi::c_void,
    fn_ptrs: std::collections::HashMap<String, *mut std::ffi::c_void>,
}
impl Library {
    pub fn try_load(name: &str) -> Result<Self, ()> {
        let ptr = unsafe { bindings::dlopen(name, 0x2) };
        if (ptr.is_null()) {
            return Err(());
        }
        if !ptr.is_aligned() {
            return Err(());
        }
        return Ok(Self {
            module: ptr,
            fn_ptrs: Default::default(),
        });
    }
    pub fn load_symbol(&self, name: &str) -> *mut std::ffi::c_void {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe { bindings::dlsym(self.module, name.as_ptr()) }
    }
}

#[derive(Debug)]
pub struct Gl {
    module: *mut std::ffi::c_void,
    fn_ptrs: std::collections::HashMap<String, *mut std::ffi::c_void>,
}

impl Gl {
    unsafe fn load() -> *mut std::ffi::c_void {
        let paths = [
            "C:\\Windows\\System32\\opengl32.dll",
            "/usr/lib/x86_64-linux-gnu/libGL.so",
            "/usr/lib/libGL.so",
            "/usr/lib/x86_64-linux-gnu/libGL.so",
        ];
        let mut ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        for path in paths {
            ptr = bindings::dlopen(path, 0x2);
            if !ptr.is_null() {
                break;
            }
        }
        return ptr;
    }
    pub fn try_load() -> Result<Self, ()> {
        let ptr = unsafe { Self::load() };
        if ptr.is_null() {
            return Err(());
        }
        if !ptr.is_aligned() {
            return Err(());
        }
        return Ok(Self {
            module: ptr,
            fn_ptrs: std::collections::HashMap::new(),
        });
    }

    pub fn platform_gl_get_proc_address(&self, name: &str) -> Result<*mut std::ffi::c_void, ()> {
        let c = std::ffi::CString::new(name).unwrap();
        let c_bytes = c.as_bytes_with_nul();
        let mut ptr = unsafe { bindings::dlsym(self.module, c"wglGetProcAddress".as_ptr()) };
        if !ptr.is_null() {
            let wgl_get_proc_address: unsafe extern "system" fn(
                *const u8,
            )
                -> *mut std::ffi::c_void = unsafe { std::mem::transmute(ptr) };
            return Ok(unsafe { wgl_get_proc_address(c_bytes.as_ptr()) });
        }
        ptr = unsafe { bindings::dlsym(self.module, c"glxGetProcAddress".as_ptr()) };
        if !ptr.is_null() {
            let glx_get_proc_address: unsafe extern "C" fn(*const u8) -> *mut std::ffi::c_void =
                unsafe { std::mem::transmute(ptr) };
            return Ok(unsafe { glx_get_proc_address(c_bytes.as_ptr()) });
        }
        ptr = unsafe { bindings::dlsym(self.module, c.as_ptr()) };
        if !ptr.is_null() {
            return Ok(ptr);
        }
        Err(())
    }
}

pub fn load_gl_function(name: &str) {}

#[cfg(unix)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session_type = std::env::var("XDG_SESSION_TYPE")?;
    if session_type != "x11" {
        return Err(
            format!("This application does not support session type '{session_type}'").into(),
        );
    }
    let display = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
    if display.is_null() {
        return Err("Failed to open a connection to an X11 Server".into());
    }
    //-- create an x11 window
    // first get the default screen
    let screen: *mut x11::xlib::Screen = unsafe { x11::xlib::XDefaultScreenOfDisplay(display) };
    let screen_id = unsafe { x11::xlib::XDefaultScreen(display) };
    // get the root window from the screen
    let root_window = unsafe { *screen }.root;

    let window_x = 0;
    let window_y = 0;
    let window_width = 800;
    let window_height = 600;
    let border_width = 0;
    let window_depth = x11::xlib::CopyFromParent;
    let window_class = x11::xlib::CopyFromParent;
    // interesting: this creates a null pointer since CopyFromParent == 0
    let visual: *mut x11::xlib::Visual = x11::xlib::CopyFromParent as *mut x11::xlib::Visual;

    let attribute_value_mask = x11::xlib::CWBackPixel | x11::xlib::CWEventMask;

    // set the window attributes
    let mut window_attributes: x11::xlib::XSetWindowAttributes = unsafe { std::mem::zeroed() };
    window_attributes.background_pixel = 0xffafe9af;
    window_attributes.event_mask = x11::xlib::ExposureMask
        | x11::xlib::KeyPressMask
        | x11::xlib::KeyReleaseMask
        | x11::xlib::ExposureMask
        | x11::xlib::StructureNotifyMask;

    // create the window
    let window: x11::xlib::Window = unsafe {
        x11::xlib::XCreateWindow(
            display,
            root_window,
            window_x,
            window_y,
            window_width,
            window_height,
            border_width,
            window_depth,
            window_class as u32,
            visual,
            attribute_value_mask,
            &raw mut window_attributes,
        )
    };

    // set the name of the window
    unsafe { x11::xlib::XStoreName(display, window, c"A Game Window".as_ptr()) };

    // register WM_DELETE_WINDOW atom to handle window close
    let mut wm_delete_window: x11::xlib::Atom =
        unsafe { x11::xlib::XInternAtom(display, c"WM_DELETE_WINDOW".as_ptr(), x11::xlib::False) };

    if unsafe { x11::xlib::XSetWMProtocols(display, window, &raw mut wm_delete_window, 1) } == 0 {
        println!("Possibly unable to set the WM_DELETE_WINDOW property for  the main window. the app may not exit properly")
    }

    // make it visible
    unsafe { x11::xlib::XMapRaised(display, window) };

    let mut event: x11::xlib::XEvent = unsafe { std::mem::zeroed() };
    let mut window_is_open = true;
    while window_is_open {
        // process window events
        while unsafe { x11::xlib::XPending(display) } > 0 {
            unsafe { x11::xlib::XNextEvent(display, &raw mut event) };
            match event.get_type() {
                x11::xlib::ClientMessage => {
                    let client_message = x11::xlib::XClientMessageEvent::from(event);
                    // check for the close button
                    if (client_message.message_type
                        == unsafe {
                            x11::xlib::XInternAtom(
                                display,
                                c"WM_PROTOCOLS".as_ptr(),
                                x11::xlib::False,
                            )
                        })
                        && client_message.data.get_long(0) == wm_delete_window.try_into().unwrap()
                    {
                        unsafe {
                            x11::xlib::XDestroyWindow(display, window);
                        }
                        window_is_open = false;
                    } else {
                        println!("got client message {client_message:?}");
                    }
                }
                x11::xlib::Expose => {
                    let expose_event = x11::xlib::XExposeEvent::from(event);
                    println!("Got expose event {expose_event:?}");
                }
                x11::xlib::ConfigureNotify => {
                    let configure_event = x11::xlib::XConfigureEvent::from(event);
                    println!("got configure event {configure_event:?}")
                }
                x11::xlib::ReparentNotify => {
                    // do nothing for now
                }
                unhandled => {
                    dbg!(unhandled);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(50));
    }
    println!("window is closed. cleaning up");
    unsafe {
        x11::xlib::XCloseDisplay(display);
    }
    Ok(())
}
#[cfg(all(not(unix), target_os = "windows"))]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
#[cfg(all(not(unix), not(target_os = "windows")))]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_error!("This application is not supported on this platform")
}

// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let gl = Gl::try_load().map_err(|e| "Failed to load GL")?;

//     // create a window using platform specific apis
//     dbg!(gl);
//     #[cfg(unix)]
//     {
//         let xlib = bindings::unix::XLibLibrary::try_load().unwrap();
//         let display = xlib.x_open_display();
//         let screen = xlib.x_default_screen_of_display(display);
//         dbg!(unsafe {&*screen});
//         dbg!(display,screen);
//     }

//     Ok(())
// }
