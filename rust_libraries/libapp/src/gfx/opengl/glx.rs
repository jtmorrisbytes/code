
use std::i32;

use winit::raw_window_handle::XlibDisplayHandle;

use crate::gfx::opengl::const_glubyte_ptr_to_string;
use crate::gfx::opengl::LoadGLFunctionAsMutCVoid;

use super::bindings;
use super::Library;

#[link(name="libGL",kind="dylib")]
unsafe extern "C" {
    unsafe fn glXGetProcAddress(procName: *const bindings::gl::GLubyte) -> *mut std::ffi::c_void;

}
pub fn glx_get_proc_address(name: &str) -> Result<*mut std::ffi::c_void,()> {
    let cstring = std::ffi::CString::new(name).unwrap_or_default();
    let bytes = cstring.as_bytes_with_nul();
    let ptr = unsafe {glXGetProcAddress(bytes.as_ptr())};
    if !ptr.is_aligned() || ptr.is_null() {
        return Err(())
    }
    else {
        Ok(ptr)
    }
}

#[derive(Debug,PartialEq)]
pub enum GLXVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
}
impl GLXVersion {
    fn try_new(major: i32,minor:i32) -> Result<Self,()> {
    let _self =    match (major, minor) {
        (1, 0) => GLXVersion::V1_0,
        (1, 1) => GLXVersion::V1_1,
        (1, 2) => GLXVersion::V1_2,
        (1, 3) => GLXVersion::V1_3,
        (1, 4) => GLXVersion::V1_4,
        _ => return Err(()),
    };
    Ok(_self)
    }
}
#[derive(Debug)]
pub struct GLXContext {
    library: Library,
    glxcontext: bindings::glx::types::GLXContext,
    glx_loader: bindings::glx::Glx,
    display: *mut bindings::glx::types::Display,
    window: bindings::glx::types::Window,
    version: GLXVersion,
}
impl GLXContext {
    pub fn is_direct(&self) -> bool {
            unsafe {self.glx_loader.IsDirect(self.display, self.glxcontext.clone()) != 0}
    }
    pub fn make_current(&self) -> bool {
         unsafe { self.glx_loader.MakeCurrent(self.display, self.window, self.glxcontext.clone()) != 0 }
    }
    pub fn get_glx_version(&self) -> Result<GLXVersion,()> {
        let mut major= 0;
        let mut minor = 0;
        let result = unsafe {self.glx_loader.QueryVersion(self.display, &mut major, &mut minor)};
        if result !=1 {
            return Err(())
        }
        else {
            GLXVersion::try_new(major, minor)
        }
    }

    pub fn get_gl_version(&self) -> Result<String,()> {

        let gl = bindings::gl::v1_0::Gl::load_with(|name| self.library.load_symbol(name));
        let s = unsafe { gl.GetString(bindings::gl::v1_1::VERSION) };
        if s.is_null() {
            return Err(());
        }
        if !s.is_aligned() {
            return Err(());
        }
        let s = unsafe { const_glubyte_ptr_to_string(s) };
        Ok(s)
    }
    pub fn draw(&mut self) {}
}
// **** GLX UTILS ******
// loads the appropriate GL library for X11, attempts to use glx v1.0 to get the supported version, then returns it

pub fn glx_get_version_from_xlib(
    glx: &bindings::glx::Glx,
    dpy: *mut bindings::glx::types::Display,
) -> Result<GLXVersion,()> {
    let mut major: i32 = 0;
    let mut minor: i32 = 0;
    let result = unsafe { glx.QueryVersion(dpy, &mut major, &mut minor) };
    if result !=1 {
        return Err(())
    }
    else {
        GLXVersion::try_new(major, minor)
    }
}

pub struct GLXAttribs(Vec<std::ffi::c_int>);

pub fn glx_choose_visual(
    library: &Library,
    glx_version: &GLXVersion,
    display: *mut bindings::glx::types::Display,
    screen: *mut bindings::glx::types::Screen,
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
    match glx_version {
        GLXVersion::V1_0 => {
            attribs.push(0);
        }
        GLXVersion::V1_1 => {
            attribs.push(0);
        }
        GLXVersion::V1_2 => {
            attribs.push(0);
        }
        GLXVersion::V1_3 => {
            attribs.push(0);

        }
        GLXVersion::V1_4 => {
            attribs.push(bindings::glx::SAMPLES as i32);
            attribs.push(samples_count.try_into().unwrap_or(0));
            attribs.push(bindings::glx::SAMPLE_BUFFERS as i32);
            attribs.push(sample_buffers_count as i32);
            attribs.push(0);
        }
    }
    let glx = bindings::glx::Glx::load_with(|name| library.load_symbol(name));
    // try choosefbconfig first
    let visualinfo = unsafe { glx.ChooseVisual(display, x11::xlib::XScreenNumberOfScreen(screen), attribs_ptr) };
    if visualinfo.is_null() {
        return Err(());
    }
    Ok(visualinfo)
}
// get the supported opengl version using platform specific apis from a winit window handle
pub fn glx_create_context_from_winit_xlib_handle(
    display_handle: XlibDisplayHandle,
    window_handle: winit::raw_window_handle::XlibWindowHandle,
) -> Result<GLXContext, ()> {
    let display: *mut bindings::glx::types::Display = display_handle.display.unwrap().as_ptr().cast();
    let screen =display_handle.screen;
    let screen: *mut bindings::glx::types::Screen = unsafe {x11::xlib::XScreenOfDisplay(display.cast(), screen).cast()};
    let window = window_handle.window;
    let visual_id = window_handle.visual_id;
    glx_create_context(display, screen, window, visual_id)
}


pub fn glx_create_context(
    // glx_version: GLXVersion,
    display: *mut bindings::glx::types::Display,
    screen: *mut bindings::glx::types::Screen,
    window: bindings::glx::types::Window,
    visualid: bindings::glx::types::VisualID, // visualinfo: *mut bindings::glx::types::XVisualInfo,
) -> Result<GLXContext, ()> {
    let libgl = Library::new("/usr/lib/x86_64-linux-gnu/libGL.so");
    let glx = bindings::glx::Glx::load_with(|name| libgl.load_symbol(name));
    let glx_version = glx_get_version_from_xlib(&glx, display)?;
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
            let glx = bindings::glx::Glx::load_with(|name| libgl.load_symbol(name));
            let visualinfo = glx_choose_visual(
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
            let ctx = unsafe { glx.CreateContext(display, visualinfo, std::ptr::null_mut(), 1) };
            if ctx.is_null() || !ctx.is_aligned() {
                return Err(());
            }
            Ok(GLXContext{
                library: libgl,
                glxcontext: ctx,
                glx_loader:glx,
                display: display,
                window: window,
                version:glx_version
            })
        }
    }
}

impl LoadGLFunctionAsMutCVoid for GLXContext {
    fn load_gl_function_as_mut_c_void(&self,name: &str) -> *mut std::ffi::c_void {
        glx_get_proc_address(name).unwrap()
    }
}
impl LoadGLFunctionAsMutCVoid for &GLXContext {
    fn load_gl_function_as_mut_c_void(&self,name: &str) -> *mut std::ffi::c_void {
        glx_get_proc_address(name).unwrap()
    }
}



pub trait GLContext {}
