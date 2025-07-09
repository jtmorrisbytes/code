
mod __gl_imports {
    pub use std::marker::Send;
    pub use std::mem;
    pub use std::os::raw;
}

pub mod types {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        dead_code,
        missing_copy_implementations
    )]
    pub use super::super::types::*;
}
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_ALPHA_SIZE: types::GLenum = 17;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_BLUE_SIZE: types::GLenum = 16;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_GREEN_SIZE: types::GLenum = 15;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_RED_SIZE: types::GLenum = 14;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_SIZE: types::GLenum = 11;
#[allow(dead_code, non_upper_case_globals)]
pub const AUX_BUFFERS: types::GLenum = 7;
#[allow(dead_code, non_upper_case_globals)]
pub const BAD_ATTRIBUTE: types::GLenum = 2;
#[allow(dead_code, non_upper_case_globals)]
pub const BAD_CONTEXT: types::GLenum = 5;
#[allow(dead_code, non_upper_case_globals)]
pub const BAD_ENUM: types::GLenum = 7;
#[allow(dead_code, non_upper_case_globals)]
pub const BAD_SCREEN: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const BAD_VALUE: types::GLenum = 6;
#[allow(dead_code, non_upper_case_globals)]
pub const BAD_VISUAL: types::GLenum = 4;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE_SIZE: types::GLenum = 10;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_SIZE: types::GLenum = 2;
#[allow(dead_code, non_upper_case_globals)]
pub const BufferSwapComplete: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_SIZE: types::GLenum = 12;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLEBUFFER: types::GLenum = 5;
#[allow(dead_code, non_upper_case_globals)]
pub const EXTENSION_NAME: &'static str = "GLX";
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN_SIZE: types::GLenum = 9;
#[allow(dead_code, non_upper_case_globals)]
pub const LEVEL: types::GLenum = 3;
#[allow(dead_code, non_upper_case_globals)]
pub const NO_EXTENSION: types::GLenum = 3;
#[allow(dead_code, non_upper_case_globals)]
pub const PbufferClobber: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const RED_SIZE: types::GLenum = 8;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA: types::GLenum = 4;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_SIZE: types::GLenum = 13;
#[allow(dead_code, non_upper_case_globals)]
pub const STEREO: types::GLenum = 6;
#[allow(dead_code, non_upper_case_globals)]
pub const USE_GL: types::GLenum = 1;

#[allow(dead_code, missing_copy_implementations)]
#[derive(Clone,Debug)]
pub struct FnPtr {
    /// The function pointer that will be used when calling the function.
    f: *const __gl_imports::raw::c_void,
    /// True if the pointer points to a real function, false if points to a `panic!` fn.
    is_loaded: bool,
}

impl FnPtr {
    /// Creates a `FnPtr` from a load attempt.
    fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
        if ptr.is_null() {
            FnPtr {
                f: missing_fn_panic as *const __gl_imports::raw::c_void,
                is_loaded: false,
            }
        } else {
            FnPtr {
                f: ptr,
                is_loaded: true,
            }
        }
    }

    /// Returns `true` if the function has been successfully loaded.
    ///
    /// If it returns `false`, calling the corresponding function will fail.
    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

#[inline(never)]
fn missing_fn_panic() -> ! {
    panic!("glx function was not loaded")
}

#[allow(non_camel_case_types, non_snake_case, dead_code)]
#[derive(Clone,Debug)]
pub struct Glx {
    pub ChooseVisual: FnPtr,
    pub CopyContext: FnPtr,
    pub CreateContext: FnPtr,
    pub CreateGLXPixmap: FnPtr,
    pub DestroyContext: FnPtr,
    pub DestroyGLXPixmap: FnPtr,
    pub GetConfig: FnPtr,
    pub GetCurrentContext: FnPtr,
    pub GetCurrentDrawable: FnPtr,
    pub IsDirect: FnPtr,
    pub MakeCurrent: FnPtr,
    pub QueryExtension: FnPtr,
    pub QueryVersion: FnPtr,
    pub SwapBuffers: FnPtr,
    pub UseXFont: FnPtr,
    pub WaitGL: FnPtr,
    pub WaitX: FnPtr,
}
impl Glx {
    /// Load each OpenGL symbol using a custom load function. This allows for the
    /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
    ///
    /// ~~~ignore
    /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
    /// ~~~
    #[allow(dead_code, unused_variables)]
    pub fn load_with<F>(mut loadfn: F) -> Glx
    where
        F: FnMut(&str) -> *const __gl_imports::raw::c_void,
    {
        #[inline(never)]
        fn do_metaloadfn(
            loadfn: &mut impl FnMut(&str) -> *const __gl_imports::raw::c_void,
            symbol: &str,
            symbols: &[&str],
        ) -> *const __gl_imports::raw::c_void {
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {
                for &sym in symbols {
                    ptr = loadfn(sym);
                    if !ptr.is_null() {
                        break;
                    }
                }
            }
            ptr
        }
        let mut metaloadfn =
            |symbol: &str, symbols: &[&str]| do_metaloadfn(&mut loadfn, symbol, symbols);
        Glx {
            ChooseVisual: FnPtr::new(metaloadfn("glXChooseVisual", &[])),
            CopyContext: FnPtr::new(metaloadfn("glXCopyContext", &[])),
            CreateContext: FnPtr::new(metaloadfn("glXCreateContext", &[])),
            CreateGLXPixmap: FnPtr::new(metaloadfn("glXCreateGLXPixmap", &[])),
            DestroyContext: FnPtr::new(metaloadfn("glXDestroyContext", &[])),
            DestroyGLXPixmap: FnPtr::new(metaloadfn("glXDestroyGLXPixmap", &[])),
            GetConfig: FnPtr::new(metaloadfn("glXGetConfig", &[])),
            GetCurrentContext: FnPtr::new(metaloadfn("glXGetCurrentContext", &[])),
            GetCurrentDrawable: FnPtr::new(metaloadfn("glXGetCurrentDrawable", &[])),
            IsDirect: FnPtr::new(metaloadfn("glXIsDirect", &[])),
            MakeCurrent: FnPtr::new(metaloadfn("glXMakeCurrent", &[])),
            QueryExtension: FnPtr::new(metaloadfn("glXQueryExtension", &[])),
            QueryVersion: FnPtr::new(metaloadfn("glXQueryVersion", &[])),
            SwapBuffers: FnPtr::new(metaloadfn("glXSwapBuffers", &[])),
            UseXFont: FnPtr::new(metaloadfn("glXUseXFont", &[])),
            WaitGL: FnPtr::new(metaloadfn("glXWaitGL", &[])),
            WaitX: FnPtr::new(metaloadfn("glXWaitX", &[])),
        }
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn ChooseVisual(
        &self,
        dpy: *mut types::Display,
        screen: __gl_imports::raw::c_int,
        attribList: *mut __gl_imports::raw::c_int,
    ) -> *mut types::XVisualInfo {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                __gl_imports::raw::c_int,
                *mut __gl_imports::raw::c_int,
            ) -> *mut types::XVisualInfo,
        >(self.ChooseVisual.f)(dpy, screen, attribList)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CopyContext(
        &self,
        dpy: *mut types::Display,
        src: types::GLXContext,
        dst: types::GLXContext,
        mask: __gl_imports::raw::c_ulong,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                types::GLXContext,
                types::GLXContext,
                __gl_imports::raw::c_ulong,
            ) -> (),
        >(self.CopyContext.f)(dpy, src, dst, mask)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CreateContext(
        &self,
        dpy: *mut types::Display,
        vis: *mut types::XVisualInfo,
        shareList: types::GLXContext,
        direct: types::Bool,
    ) -> types::GLXContext {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                *mut types::XVisualInfo,
                types::GLXContext,
                types::Bool,
            ) -> types::GLXContext,
        >(self.CreateContext.f)(dpy, vis, shareList, direct)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn CreateGLXPixmap(
        &self,
        dpy: *mut types::Display,
        visual: *mut types::XVisualInfo,
        pixmap: types::Pixmap,
    ) -> types::GLXPixmap {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                *mut types::XVisualInfo,
                types::Pixmap,
            ) -> types::GLXPixmap,
        >(self.CreateGLXPixmap.f)(dpy, visual, pixmap)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DestroyContext(&self, dpy: *mut types::Display, ctx: types::GLXContext) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(*mut types::Display, types::GLXContext) -> (),
        >(self.DestroyContext.f)(dpy, ctx)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn DestroyGLXPixmap(
        &self,
        dpy: *mut types::Display,
        pixmap: types::GLXPixmap,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(*mut types::Display, types::GLXPixmap) -> (),
        >(self.DestroyGLXPixmap.f)(dpy, pixmap)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetConfig(
        &self,
        dpy: *mut types::Display,
        visual: *mut types::XVisualInfo,
        attrib: __gl_imports::raw::c_int,
        value: *mut __gl_imports::raw::c_int,
    ) -> __gl_imports::raw::c_int {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                *mut types::XVisualInfo,
                __gl_imports::raw::c_int,
                *mut __gl_imports::raw::c_int,
            ) -> __gl_imports::raw::c_int,
        >(self.GetConfig.f)(dpy, visual, attrib, value)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetCurrentContext(&self) -> types::GLXContext {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLXContext>(
            self.GetCurrentContext.f,
        )()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn GetCurrentDrawable(&self) -> types::GLXDrawable {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLXDrawable>(
            self.GetCurrentDrawable.f,
        )()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn IsDirect(&self, dpy: *mut types::Display, ctx: types::GLXContext) -> types::Bool {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(*mut types::Display, types::GLXContext) -> types::Bool,
        >(self.IsDirect.f)(dpy, ctx)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn MakeCurrent(
        &self,
        dpy: *mut types::Display,
        drawable: types::GLXDrawable,
        ctx: types::GLXContext,
    ) -> types::Bool {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                types::GLXDrawable,
                types::GLXContext,
            ) -> types::Bool,
        >(self.MakeCurrent.f)(dpy, drawable, ctx)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn QueryExtension(
        &self,
        dpy: *mut types::Display,
        errorb: *mut __gl_imports::raw::c_int,
        event: *mut __gl_imports::raw::c_int,
    ) -> types::Bool {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                *mut __gl_imports::raw::c_int,
                *mut __gl_imports::raw::c_int,
            ) -> types::Bool,
        >(self.QueryExtension.f)(dpy, errorb, event)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn QueryVersion(
        &self,
        dpy: *mut types::Display,
        maj: *mut __gl_imports::raw::c_int,
        min: *mut __gl_imports::raw::c_int,
    ) -> types::Bool {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                *mut types::Display,
                *mut __gl_imports::raw::c_int,
                *mut __gl_imports::raw::c_int,
            ) -> types::Bool,
        >(self.QueryVersion.f)(dpy, maj, min)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn SwapBuffers(&self, dpy: *mut types::Display, drawable: types::GLXDrawable) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(*mut types::Display, types::GLXDrawable) -> (),
        >(self.SwapBuffers.f)(dpy, drawable)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn UseXFont(
        &self,
        font: types::Font,
        first: __gl_imports::raw::c_int,
        count: __gl_imports::raw::c_int,
        list: __gl_imports::raw::c_int,
    ) -> () {
        __gl_imports::mem::transmute::<
            _,
            extern "system" fn(
                types::Font,
                __gl_imports::raw::c_int,
                __gl_imports::raw::c_int,
                __gl_imports::raw::c_int,
            ) -> (),
        >(self.UseXFont.f)(font, first, count, list)
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn WaitGL(&self) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.WaitGL.f)()
    }
    #[allow(non_snake_case, unused_variables, dead_code)]
    #[inline]
    pub unsafe fn WaitX(&self) -> () {
        __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.WaitX.f)()
    }
}

unsafe impl __gl_imports::Send for Glx {}
