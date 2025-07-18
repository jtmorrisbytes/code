#[cfg(unix)]
use winit::raw_window_handle::RawDisplayHandle;

pub mod bindings;
#[cfg(unix)]
pub mod glx;
#[cfg(target_os="windows")]
pub mod wgl;

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




pub enum OpenGLContextKind {
        #[cfg(unix)]
    GLX(glx::GLXContext),
    #[cfg(target_os="windows")]
    WGL(self::wgl::WGLContext)
}

impl LoadGLFunctionAsMutCVoid for OpenGLContextKind {
    fn load_gl_function_as_mut_c_void(&self,name: &str) -> *mut std::ffi::c_void {
        match self {
            #[cfg(unix)]
            Self::GLX(ctx)=>{
                (Box::new(ctx) as Box<dyn LoadGLFunctionAsMutCVoid>).load_gl_function_as_mut_c_void(name)
            }
            #[cfg(target_os="windows")]
            Self::WGL(ctx)=>{
                (Box::new(ctx) as Box<dyn LoadGLFunctionAsMutCVoid>).load_gl_function_as_mut_c_void(name)
            }
        }
    }
}
impl LoadGLFunctionAsMutCVoid for &OpenGLContextKind {
    fn load_gl_function_as_mut_c_void(&self,name: &str) -> *mut std::ffi::c_void {
        match *self {
            #[cfg(unix)]
            OpenGLContextKind::GLX(ctx)=>{
                (Box::new(ctx) as Box<dyn LoadGLFunctionAsMutCVoid>).load_gl_function_as_mut_c_void(name)
            }
            #[cfg(target_os="windows")]
            OpenGLContextKind::WGL(ctx)=>{
                (Box::new(ctx) as Box<dyn LoadGLFunctionAsMutCVoid>).load_gl_function_as_mut_c_void(name)
            }
        }
    }
}


pub struct OpenGL1_0Context {
   kind: OpenGLContextKind,
   loader: bindings::gl::v1_0::Gl,
//    library: Library
}




pub trait LoadGLFunctionAsMutCVoid {
    fn load_gl_function_as_mut_c_void(&self,name: &str) -> *mut std::ffi::c_void;
}



impl OpenGL1_0Context {
    pub fn try_new(window_handle: winit::raw_window_handle::RawWindowHandle,display_handle: winit::raw_window_handle::RawDisplayHandle) -> Result<Self,()> {
        use winit::raw_window_handle::RawWindowHandle;
        let ctx = match (window_handle,display_handle) {
            #[cfg(unix)] 
                (RawWindowHandle::Xlib(window_handle),RawDisplayHandle::Xlib(display_handle))=>{
                    let ctx = self::glx::glx_create_context_from_winit_xlib_handle(display_handle, window_handle).unwrap();
                    OpenGLContextKind::GLX(ctx)
                }
                #[cfg(target_os="windows")]
                (RawWindowHandle::Win32(window_handle),_) => {
                    let hwnd = windows::Win32::Foundation::HWND(window_handle.hwnd.get() as *mut std::ffi::c_void);
                    if hwnd.is_invalid() {
                        panic!("invalid window handle")
                    }
                    let dc = self::wgl::get_device_context(Some(hwnd)).unwrap();
                    let ctx = self::wgl::create_context(hwnd,Some(dc)).unwrap();
                    OpenGLContextKind::WGL(ctx)
                }
                unimplemented=> {
                    todo!("{unimplemented:?}")
                }
                
            };
            let gl = bindings::gl::v1_0::Gl::load_with(|name| (Box::new(&ctx) as Box<dyn LoadGLFunctionAsMutCVoid>).load_gl_function_as_mut_c_void(name));
            Ok(Self{
                kind:ctx,
                loader:gl
            })

    }
    pub fn make_current(&self) {
        match self.kind {
            #[cfg(unix)]
            OpenGLContextKind::GLX(ref ctx) => {ctx.make_current();},
            #[cfg(target_os="windows")]
            OpenGLContextKind::WGL(ref ctx) => {
                ctx.make_current();
            }
        }
    }
    pub fn get_opengl_version_string(&self) -> String {
                match self.kind {
            #[cfg(unix)]
            OpenGLContextKind::GLX(ref ctx) => {ctx.get_gl_version().unwrap()},
            #[cfg(target_os="windows")]
            OpenGLContextKind::WGL(ref ctx) => {
                ctx.get_gl_version();
                String::new()
            }
        }
    }
    pub fn gl_get_error(&self) -> bindings::gl::v1_0::types::GLenum{
        unsafe {self.loader.GetError()}
    }
    pub fn gl_begin(&self, mode: bindings::gl::v1_0::types::GLenum) -> Result<(),bindings::gl::v1_0::types::GLenum>{
        // clear the error flag
        let _ = self.gl_get_error();
        unsafe {self.loader.Begin(mode)}
        let err = self.gl_get_error();
        if err == 0 {
            return Ok(())
        }
        else {return Err(err);}
    }
    pub fn gl_end(&self) -> Result<(),bindings::gl::v1_0::types::GLenum> {
        // clear the error flag
        let _ = self.gl_get_error();
        // call the function
        unsafe {self.loader.End()}
        let err = self.gl_get_error();
        if err == 0 {
            return Ok(())
        }
        else {return Err(err)}
    }
    pub fn gl_clear_color(&self,red:bindings::gl::v1_0::types::GLfloat,green:bindings::gl::v1_0::types::GLfloat,blue:bindings::gl::v1_0::types::GLfloat,alpha:bindings::gl::v1_0::types::GLfloat) {
        unsafe {self.loader.ClearColor(red, green, blue, alpha);}
    }
    pub fn gl_clear(&self,mask: bindings)
}
