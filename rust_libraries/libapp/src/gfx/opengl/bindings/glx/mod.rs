pub mod v1_0;
pub mod v1_1;
pub mod v1_2;
pub mod v1_3;
pub mod v1_4;

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
    // Common types from OpenGL 1.1
    pub type GLenum = super::__gl_imports::raw::c_uint;
    pub type GLboolean = super::__gl_imports::raw::c_uchar;
    pub type GLbitfield = super::__gl_imports::raw::c_uint;
    pub type GLvoid = super::__gl_imports::raw::c_void;
    pub type GLbyte = super::__gl_imports::raw::c_char;
    pub type GLshort = super::__gl_imports::raw::c_short;
    pub type GLint = super::__gl_imports::raw::c_int;
    pub type GLclampx = super::__gl_imports::raw::c_int;
    pub type GLubyte = super::__gl_imports::raw::c_uchar;
    pub type GLushort = super::__gl_imports::raw::c_ushort;
    pub type GLuint = super::__gl_imports::raw::c_uint;
    pub type GLsizei = super::__gl_imports::raw::c_int;
    pub type GLfloat = super::__gl_imports::raw::c_float;
    pub type GLclampf = super::__gl_imports::raw::c_float;
    pub type GLdouble = super::__gl_imports::raw::c_double;
    pub type GLclampd = super::__gl_imports::raw::c_double;
    pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
    pub type GLchar = super::__gl_imports::raw::c_char;
    pub type GLcharARB = super::__gl_imports::raw::c_char;
    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = super::__gl_imports::raw::c_uint;

    pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
    pub type GLhalf = super::__gl_imports::raw::c_ushort;

    // Must be 32 bits
    pub type GLfixed = GLint;

    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;

    pub enum __GLsync {}
    pub type GLsync = *const __GLsync;

    // compatible with OpenCL cl_context
    pub enum _cl_context {}
    pub enum _cl_event {}

    pub type GLDEBUGPROC = extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut super::__gl_imports::raw::c_void,
    );
    pub type GLDEBUGPROCARB = extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut super::__gl_imports::raw::c_void,
    );
    pub type GLDEBUGPROCKHR = extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut super::__gl_imports::raw::c_void,
    );

    // GLES 1 types
    // "pub type GLclampx = i32;",

    // GLES 1/2 types (tagged for GLES 1)
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",

    // GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLint64EXT = i64;",
    // "pub type GLuint64EXT = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",

    // GLES 2 types (none currently)

    // Vendor extension types
    pub type GLDEBUGPROCAMD = extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut super::__gl_imports::raw::c_void,
    );
    pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
    pub type GLvdpauSurfaceNV = GLintptr;

    pub type XID = super::__gl_imports::raw::c_ulong;
    pub type Bool = super::__gl_imports::raw::c_int; // Not sure if this is correct...
    pub type Screen = super::__gl_imports::raw::c_int;
    pub enum Display {}

    pub type Font = XID;
    pub type Pixmap = XID;
    #[derive(Debug)]
    pub enum Visual {} // TODO: not sure
    pub type VisualID = XID; // TODO: not sure
    pub type Window = XID;
    pub type GLXFBConfigID = XID;
    pub type GLXFBConfig = *const super::__gl_imports::raw::c_void;
    pub type GLXContextID = XID;
    pub type GLXContext = *const super::__gl_imports::raw::c_void;
    pub type GLXPixmap = XID;
    pub type GLXDrawable = XID;
    pub type GLXWindow = XID;
    pub type GLXPbuffer = XID;
    pub type __GLXextFuncPtr = extern "system" fn();
    pub type GLXVideoCaptureDeviceNV = XID;
    pub type GLXVideoDeviceNV = super::__gl_imports::raw::c_int;
    pub type GLXVideoSourceSGIX = XID;
    pub type GLXFBConfigIDSGIX = XID;
    pub type GLXFBConfigSGIX = *const super::__gl_imports::raw::c_void;
    pub type GLXPbufferSGIX = XID;


    pub type XVisualInfoScreen = super::__gl_imports::raw::c_int;
    pub type XVisualInfoDepth = super::__gl_imports::raw::c_int;
    pub type XVisualInfoClass = super::__gl_imports::raw::c_int;
    pub type XVisualInfoMask = super::__gl_imports::raw::c_ulong;
    pub type XVisualInfoColormapSize = super::__gl_imports::raw::c_int;
    pub type XVisualInfoBitsPerRgb = super::__gl_imports::raw::c_int;

    #[repr(C)]
    #[derive(Debug)]
    pub struct XVisualInfo {
        pub visual: *mut Visual,
        pub visualid: VisualID,
        pub screen: XVisualInfoScreen,
        pub depth: XVisualInfoDepth,
        pub class: XVisualInfoClass,
        pub red_mask: XVisualInfoMask,
        pub green_mask: XVisualInfoMask,
        pub blue_mask: XVisualInfoMask,
        pub colormap_size: XVisualInfoColormapSize,
        pub bits_per_rgb: XVisualInfoBitsPerRgb,
    }
    #[repr(C)]
    pub struct GLXPbufferClobberEvent {
        pub event_type: super::__gl_imports::raw::c_int, // GLX_DAMAGED or GLX_SAVED
        pub draw_type: super::__gl_imports::raw::c_int,  // GLX_WINDOW or GLX_PBUFFER
        pub serial: super::__gl_imports::raw::c_ulong,   // # of last request processed by server
        pub send_event: Bool,                            // true if this came for SendEvent request
        pub display: *const Display,                     // display the event was read from
        pub drawable: GLXDrawable,                       // XID of Drawable
        pub buffer_mask: super::__gl_imports::raw::c_uint, // mask indicating which buffers are affected
        pub aux_buffer: super::__gl_imports::raw::c_uint,  // which aux buffer was affected
        pub x: super::__gl_imports::raw::c_int,
        pub y: super::__gl_imports::raw::c_int,
        pub width: super::__gl_imports::raw::c_int,
        pub height: super::__gl_imports::raw::c_int,
        pub count: super::__gl_imports::raw::c_int, // if nonzero, at least this many more
    }
    #[repr(C)]
    pub struct GLXBufferSwapComplete {
        pub type_: super::__gl_imports::raw::c_int,
        pub serial: super::__gl_imports::raw::c_ulong, // # of last request processed by server
        pub send_event: Bool,                          // true if this came from a SendEvent request
        pub display: *const Display,                   // Display the event was read from
        pub drawable: GLXDrawable, // drawable on which event was requested in event mask
        pub event_type: super::__gl_imports::raw::c_int,
        pub ust: i64,
        pub msc: i64,
        pub sbc: i64,
    }
    #[repr(C)]
    pub struct GLXBufferClobberEventSGIX {
        pub type_: super::__gl_imports::raw::c_int,
        pub serial: super::__gl_imports::raw::c_ulong, // # of last request processed by server
        pub send_event: Bool,                          // true if this came for SendEvent request
        pub display: *const Display,                   // display the event was read from
        pub drawable: GLXDrawable,                     // i.d. of Drawable
        pub event_type: super::__gl_imports::raw::c_int, // GLX_DAMAGED_SGIX or GLX_SAVED_SGIX
        pub draw_type: super::__gl_imports::raw::c_int, // GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX
        pub mask: super::__gl_imports::raw::c_uint,    // mask indicating which buffers are affected
        pub x: super::__gl_imports::raw::c_int,
        pub y: super::__gl_imports::raw::c_int,
        pub width: super::__gl_imports::raw::c_int,
        pub height: super::__gl_imports::raw::c_int,
        pub count: super::__gl_imports::raw::c_int, // if nonzero, at least this many more
    }

    #[repr(C)]
    pub struct GLXHyperpipeNetworkSGIX {
        pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
        pub networkId: super::__gl_imports::raw::c_int,
    }

    #[repr(C)]
    pub struct GLXHyperpipeConfigSGIX {
        pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
        pub channel: super::__gl_imports::raw::c_int,
        pub participationType: super::__gl_imports::raw::c_uint,
        pub timeSlice: super::__gl_imports::raw::c_int,
    }

    #[repr(C)]
    pub struct GLXPipeRect {
        pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
        pub srcXOrigin: super::__gl_imports::raw::c_int,
        pub srcYOrigin: super::__gl_imports::raw::c_int,
        pub srcWidth: super::__gl_imports::raw::c_int,
        pub srcHeight: super::__gl_imports::raw::c_int,
        pub destXOrigin: super::__gl_imports::raw::c_int,
        pub destYOrigin: super::__gl_imports::raw::c_int,
        pub destWidth: super::__gl_imports::raw::c_int,
        pub destHeight: super::__gl_imports::raw::c_int,
    }

    #[repr(C)]
    pub struct GLXPipeRectLimits {
        pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
        pub XOrigin: super::__gl_imports::raw::c_int,
        pub YOrigin: super::__gl_imports::raw::c_int,
        pub maxHeight: super::__gl_imports::raw::c_int,
        pub maxWidth: super::__gl_imports::raw::c_int,
    }
}
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_ALPHA_SIZE: types::GLenum = 17;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_BLUE_SIZE: types::GLenum = 16;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_GREEN_SIZE: types::GLenum = 15;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_RED_SIZE: types::GLenum = 14;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_SIZE: types::GLenum = 11;
#[allow(dead_code, non_upper_case_globals)] pub const AUX_BUFFERS: types::GLenum = 7;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_ATTRIBUTE: types::GLenum = 2;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_CONTEXT: types::GLenum = 5;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_ENUM: types::GLenum = 7;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_SCREEN: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_VALUE: types::GLenum = 6;
#[allow(dead_code, non_upper_case_globals)] pub const BAD_VISUAL: types::GLenum = 4;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_SIZE: types::GLenum = 10;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 2;
#[allow(dead_code, non_upper_case_globals)] pub const BufferSwapComplete: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_SIZE: types::GLenum = 12;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLEBUFFER: types::GLenum = 5;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSION_NAME: &'static str = "GLX";
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_SIZE: types::GLenum = 9;
#[allow(dead_code, non_upper_case_globals)] pub const LEVEL: types::GLenum = 3;
#[allow(dead_code, non_upper_case_globals)] pub const NO_EXTENSION: types::GLenum = 3;
#[allow(dead_code, non_upper_case_globals)] pub const PbufferClobber: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const RED_SIZE: types::GLenum = 8;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_SIZE: types::GLenum = 13;
#[allow(dead_code, non_upper_case_globals)] pub const STEREO: types::GLenum = 6;
#[allow(dead_code, non_upper_case_globals)] pub const USE_GL: types::GLenum = 1;