use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::OpenGL::{
    ChoosePixelFormat, PFD_FLAGS, PFD_PIXEL_TYPE, PIXELFORMATDESCRIPTOR,
    PFD_DRAW_TO_WINDOW,PFD_SUPPORT_OPENGL, PFD_DOUBLEBUFFER,PFD_TYPE_RGBA,
    PFD_MAIN_PLANE,HGLRC
};
use windows::Win32::Foundation::HWND;

// required!







/// gets the device context associated with the handle. if the handle is none, gets the device context for the whole screen
pub fn get_device_context(hwnd:Option<HWND>) -> windows::core::Result<HDC> {
    let dc = unsafe {windows::Win32::Graphics::Gdi::GetDC(hwnd)};
    if dc.is_invalid() {
        return Err(windows::core::Error::from_win32())
    }
    Ok(dc)
}


/// A 'safe' wrapper around Win32::Graphics::OpenGL::ChoosePixelFormat and checks for errors
pub fn choose_pixel_format(
    hdc: HDC,
    pixel_format_descriptor: &PIXELFORMATDESCRIPTOR,
) -> windows::core::Result<i32> {
    let pixel_format_index = unsafe { ChoosePixelFormat(hdc, &raw const *pixel_format_descriptor) };
    if pixel_format_index == 0 {
        return Err(windows::core::Error::from_win32());
    }
    Ok(pixel_format_index)
}

pub fn set_pixel_format(handle_to_device_context: HDC,format: i32,pixel_format_descriptor: &PIXELFORMATDESCRIPTOR) -> windows::core::Result<()> {
    unsafe {windows::Win32::Graphics::OpenGL::SetPixelFormat(handle_to_device_context,format,&raw const *pixel_format_descriptor)}
}

pub fn describe_pixel_format(handle_to_device_context: HDC, i_pixel_format: i32) -> windows::core::Result<PIXELFORMATDESCRIPTOR> {
    let mut pixel_format: PIXELFORMATDESCRIPTOR = unsafe {std::mem::zeroed()};

    let result = unsafe {windows::Win32::Graphics::OpenGL::DescribePixelFormat(handle_to_device_context,i_pixel_format,std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32,Some(std::ptr::from_mut(&mut pixel_format)))};
    if result == 0 { 
        Err(windows::core::Error::from_win32())
    }
    else {
        Ok(pixel_format)
    }
}

// accepts arguments for PIXELFORMATDESCRIPTOR and creates an instance of that type on the rust side
pub fn create_pixel_format_descriptor(
    version: u16,
    dw_flags: PFD_FLAGS,
    i_pixel_type: PFD_PIXEL_TYPE,
    c_color_bits: u8,
    c_red_bits: u8,
    c_green_bits: u8,
    c_green_shift: u8,
    c_blue_bits: u8,
    c_blue_shift: u8,
    c_alpha_bits: u8,
    c_alpha_shift: u8,
    c_accum_bits: u8,
    c_accum_red_bits: u8,
    c_accum_green_bits: u8,
    c_accum_blue_bits: u8,
    c_accum_alpha_bits: u8,
    c_depth_bits: u8,
    c_stencil_bits: u8,
    c_aux_buffers: u8,
    i_layer_type: u8,
    b_reserved: u8,
    dw_layer_mask: u32,
    dw_visible_mask: u32,
    dw_damage_mask: u32,
) -> PIXELFORMATDESCRIPTOR {
    let mut pfd: PIXELFORMATDESCRIPTOR = unsafe { std::mem::zeroed() };
    
    pfd.nSize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
    pfd.nVersion = version;
    pfd.dwFlags = dw_flags;
    pfd.iPixelType = i_pixel_type;
    pfd.cColorBits = c_color_bits;
    pfd.cRedBits = c_red_bits;
    pfd.cGreenBits = c_green_bits;
    pfd.cGreenShift = c_green_shift;
    pfd.cBlueBits = c_blue_bits;
    pfd.cBlueShift = c_blue_shift;
    pfd.cAlphaBits = c_alpha_bits;
    pfd.cAlphaShift = c_alpha_shift;
    pfd.cAccumBits = c_accum_bits;
    pfd.cAccumRedBits = c_accum_red_bits;
    pfd.cAccumGreenBits = c_accum_green_bits;
    pfd.cAccumBlueBits = c_accum_blue_bits;
    pfd.cAccumAlphaBits = c_accum_alpha_bits;
    pfd.cDepthBits = c_depth_bits;
    pfd.cStencilBits = c_stencil_bits;
    pfd.cAuxBuffers = c_aux_buffers;
    pfd.iLayerType = i_layer_type;
    pfd.bReserved = b_reserved;
    pfd.dwLayerMask = dw_layer_mask;
    pfd.dwVisibleMask = dw_visible_mask;
    pfd.dwDamageMask = dw_damage_mask;
    
    pfd
}

pub fn wgl_create_context(handle_to_device_context: HDC) -> windows::core::Result<HGLRC> {
    unsafe{windows::Win32::Graphics::OpenGL::wglCreateContext(handle_to_device_context)}
}

pub fn wgl_make_current(hdc:HDC,hglrc:HGLRC) -> windows::core::Result<()> {
    if hdc.is_invalid() {
        panic!()
    }
    if hglrc.is_invalid() {
        panic!()
    }
    unsafe{windows::Win32::Graphics::OpenGL::wglMakeCurrent(hdc,hglrc)}
}


pub struct WGLContext{
    glcontext: HGLRC,
    hwnd:HWND,
    hdc: HDC,
    pfd: PIXELFORMATDESCRIPTOR
}
impl WGLContext {
    pub fn make_current(&self) -> windows::core::Result<()> {
        wgl_make_current(self.hdc,self.glcontext)
    }
    pub fn get_gl_version(&self) {
        
    }
    pub unsafe fn load_gl_function(name:&str) -> *mut std::ffi::c_void {
        let cstring = std::ffi::CString::new(name).unwrap();
        let bytes = cstring.as_bytes_with_nul();
        let ptr = windows::core::PCSTR::from_raw(bytes.as_ptr());
        let module = windows::Win32::Graphics::OpenGL::wglGetProcAddress(ptr);
        std::mem::transmute(module)
    } 
}

impl super::LoadGLFunctionAsMutCVoid for WGLContext {
    fn load_gl_function_as_mut_c_void(&self,name: &str) -> *mut std::ffi::c_void {
        unsafe {Self::load_gl_function(name)}
    }
}

impl super::LoadGLFunctionAsMutCVoid for &WGLContext {
    fn load_gl_function_as_mut_c_void(&self,name: &str) -> *mut std::ffi::c_void {
        unsafe {WGLContext::load_gl_function(name) }
    }
}

pub fn create_context(handle_to_window: HWND,hdc: Option<HDC>) -> windows::core::Result<WGLContext> {
    if handle_to_window.is_invalid() {
        panic!("Invalid hwnd")
    }

    // first choose a pixel format that describes how you want to use opengl
    let device_context = hdc.unwrap_or_else(|| get_device_context(Some(handle_to_window)).unwrap());
    let pfd: PIXELFORMATDESCRIPTOR = create_pixel_format_descriptor(1,PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,PFD_TYPE_RGBA,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,PFD_MAIN_PLANE.0 as u8,0,0,0,0);
    let pixel_format_index = choose_pixel_format(device_context,&pfd)?;
    let pixel_format = describe_pixel_format(device_context,pixel_format_index)?;
    set_pixel_format(device_context,pixel_format_index,&pixel_format)?;
    let hglrc = wgl_create_context(device_context)?;
    Ok(WGLContext{glcontext:hglrc,hwnd:handle_to_window,hdc:device_context,pfd})
}



