#![allow(non_camel_case_types)]
/*
 * Mesa 3-D graphics library
 *
 * Copyright (C) 1999-2006  Brian Paul   All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */
// use x11::glx::

// #ifndef GLX_H
// #define GLX_H

// #include <X11/Xlib.h>
// #include <X11/Xutil.h>
// #include <GL/gl.h>

// #if defined(USE_MGL_NAMESPACE)
// #include "glx_mangle.h"
// #endif

const GLX_VERSION_1_1: std::ffi::c_uint = 1;
const GLX_VERSION_1_2: std::ffi::c_uint = 1;
const GLX_VERSION_1_3: std::ffi::c_uint = 1;
const GLX_VERSION_1_4: std::ffi::c_uint = 1;

const GLX_EXTENSION_NAME: &std::ffi::CStr = c"GLX";

/*
 * Tokens for glXChooseVisual and glXGetConfig:
 */
const GLX_USE_GL: std::ffi::c_uint = 1;
const GLX_BUFFER_SIZE: std::ffi::c_uint = 2;
const GLX_LEVEL: std::ffi::c_uint = 3;
const GLX_RGBA: std::ffi::c_uint = 4;
const GLX_DOUBLEBUFFER: std::ffi::c_uint = 5;
const GLX_STEREO: std::ffi::c_uint = 6;
const GLX_AUX_BUFFERS: std::ffi::c_uint = 7;
const GLX_RED_SIZE: std::ffi::c_uint = 8;
const GLX_GREEN_SIZE: std::ffi::c_uint = 9;
const GLX_BLUE_SIZE: std::ffi::c_uint = 10;
const GLX_ALPHA_SIZE: std::ffi::c_uint = 11;
const GLX_DEPTH_SIZE: std::ffi::c_uint = 12;
const GLX_STENCIL_SIZE: std::ffi::c_uint = 13;
const GLX_ACCUM_RED_SIZE: std::ffi::c_uint = 14;
const GLX_ACCUM_GREEN_SIZE: std::ffi::c_uint = 15;
const GLX_ACCUM_BLUE_SIZE: std::ffi::c_uint = 16;
const GLX_ACCUM_ALPHA_SIZE: std::ffi::c_uint = 17;

/*
 * Error codes returned by glXGetConfig:
 */
pub const GLX_BAD_SCREEN: std::ffi::c_uint = 1;
pub const GLX_BAD_ATTRIBUTE: std::ffi::c_uint = 2;
pub const GLX_NO_EXTENSION: std::ffi::c_uint = 3;
pub const GLX_BAD_VISUAL: std::ffi::c_uint = 4;
pub const GLX_BAD_CONTEXT: std::ffi::c_uint = 5;
pub const GLX_BAD_VALUE: std::ffi::c_uint = 6;
pub const GLX_BAD_ENUM: std::ffi::c_uint = 7;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GLXGetConfigError {
    GLX_BAD_SCREEN = 1,
    GLX_BAD_ATTRIBUTE = 2,
    GLX_NO_EXTENSION = 3,
    GLX_BAD_VISUAL = 4,
    GLX_BAD_CONTEXT = 5,
    GLX_BAD_VALUE = 6,
    GLX_BAD_ENUM = 7,
}

/*
 * GLX 1.1 and later:
 */
const GLX_VENDOR: std::ffi::c_uint = 1;
const GLX_VERSION: std::ffi::c_uint = 2;
const GLX_EXTENSIONS: std::ffi::c_uint = 3;

/*
 * GLX 1.3 and later:
 */
const GLX_CONFIG_CAVEAT: std::ffi::c_uint = 0x20;
const GLX_DONT_CARE: std::ffi::c_uint = 0xFFFFFFFF;
const GLX_X_VISUAL_TYPE: std::ffi::c_uint = 0x22;
const GLX_TRANSPARENT_TYPE: std::ffi::c_uint = 0x23;
const GLX_TRANSPARENT_INDEX_VALUE: std::ffi::c_uint = 0x24;
const GLX_TRANSPARENT_RED_VALUE: std::ffi::c_uint = 0x25;
const GLX_TRANSPARENT_GREEN_VALUE: std::ffi::c_uint = 0x26;
const GLX_TRANSPARENT_BLUE_VALUE: std::ffi::c_uint = 0x27;
const GLX_TRANSPARENT_ALPHA_VALUE: std::ffi::c_uint = 0x28;
const GLX_WINDOW_BIT: std::ffi::c_uint = 0x00000001;
const GLX_PIXMAP_BIT: std::ffi::c_uint = 0x00000002;
const GLX_PBUFFER_BIT: std::ffi::c_uint = 0x00000004;
const GLX_AUX_BUFFERS_BIT: std::ffi::c_uint = 0x00000010;
const GLX_FRONT_LEFT_BUFFER_BIT: std::ffi::c_uint = 0x00000001;
const GLX_FRONT_RIGHT_BUFFER_BIT: std::ffi::c_uint = 0x00000002;
const GLX_BACK_LEFT_BUFFER_BIT: std::ffi::c_uint = 0x00000004;
const GLX_BACK_RIGHT_BUFFER_BIT: std::ffi::c_uint = 0x00000008;
const GLX_DEPTH_BUFFER_BIT: std::ffi::c_uint = 0x00000020;
const GLX_STENCIL_BUFFER_BIT: std::ffi::c_uint = 0x00000040;
const GLX_ACCUM_BUFFER_BIT: std::ffi::c_uint = 0x00000080;
const GLX_NONE: std::ffi::c_uint = 0x8000;
const GLX_SLOW_CONFIG: std::ffi::c_uint = 0x8001;
const GLX_TRUE_COLOR: std::ffi::c_uint = 0x8002;
const GLX_DIRECT_COLOR: std::ffi::c_uint = 0x8003;
const GLX_PSEUDO_COLOR: std::ffi::c_uint = 0x8004;
const GLX_STATIC_COLOR: std::ffi::c_uint = 0x8005;
const GLX_GRAY_SCALE: std::ffi::c_uint = 0x8006;
const GLX_STATIC_GRAY: std::ffi::c_uint = 0x8007;
const GLX_TRANSPARENT_RGB: std::ffi::c_uint = 0x8008;
const GLX_TRANSPARENT_INDEX: std::ffi::c_uint = 0x8009;
const GLX_VISUAL_ID: std::ffi::c_uint = 0x800B;
const GLX_SCREEN: std::ffi::c_uint = 0x800C;
const GLX_NON_CONFORMANT_CONFIG: std::ffi::c_uint = 0x800D;
const GLX_DRAWABLE_TYPE: std::ffi::c_uint = 0x8010;
const GLX_RENDER_TYPE: std::ffi::c_uint = 0x8011;
const GLX_X_RENDERABLE: std::ffi::c_uint = 0x8012;
const GLX_FBCONFIG_ID: std::ffi::c_uint = 0x8013;
const GLX_RGBA_TYPE: std::ffi::c_uint = 0x8014;
const GLX_COLOR_INDEX_TYPE: std::ffi::c_uint = 0x8015;
const GLX_MAX_PBUFFER_WIDTH: std::ffi::c_uint = 0x8016;
const GLX_MAX_PBUFFER_HEIGHT: std::ffi::c_uint = 0x8017;
const GLX_MAX_PBUFFER_PIXELS: std::ffi::c_uint = 0x8018;
const GLX_PRESERVED_CONTENTS: std::ffi::c_uint = 0x801B;
const GLX_LARGEST_PBUFFER: std::ffi::c_uint = 0x801C;
const GLX_WIDTH: std::ffi::c_uint = 0x801D;
const GLX_HEIGHT: std::ffi::c_uint = 0x801E;
const GLX_EVENT_MASK: std::ffi::c_uint = 0x801F;
const GLX_DAMAGED: std::ffi::c_uint = 0x8020;
const GLX_SAVED: std::ffi::c_uint = 0x8021;
const GLX_WINDOW: std::ffi::c_uint = 0x8022;
const GLX_PBUFFER: std::ffi::c_uint = 0x8023;
const GLX_PBUFFER_HEIGHT: std::ffi::c_uint = 0x8040;
const GLX_PBUFFER_WIDTH: std::ffi::c_uint = 0x8041;
const GLX_RGBA_BIT: std::ffi::c_uint = 0x00000001;
const GLX_COLOR_INDEX_BIT: std::ffi::c_uint = 0x00000002;
const GLX_PBUFFER_CLOBBER_MASK: std::ffi::c_uint = 0x08000000;

/*
 * GLX 1.4 and later:
 */
const GLX_SAMPLE_BUFFERS: std::ffi::c_uint = 0x186a0; /*100000*/
const GLX_SAMPLES: std::ffi::c_uint = 0x186a1; /*100001*/

use std::ffi::os_str::Display;

// what type is glxContext?
use x11::glx::GLXContext;

// typedef struct __GLXcontextRec *GLXContext;

type GLXPixmap = x11::xlib::XID;
type GLXDrawable = x11::xlib::XID;

/* GLX 1.3 and later */
use x11::glx::GLXFBConfig;
use x11::xlib::Bool;
// use x11::glx::GLXPixmap;
use x11::xlib::Pixmap;
use x11::xlib::XVisualInfo;

type GLXFBConfigID = x11::xlib::XID;
type GLXContextID = x11::xlib::XID;
type GLXWindow = x11::xlib::XID;
type GLXPbuffer = x11::xlib::XID;

/*
** Events.
** __GLX_NUMBER_EVENTS is set to 17 to account for the BufferClobberSGIX
**  event - this helps initialization if the server supports the pbuffer
**  extension and the client doesn't.
*/

const GLX_PbufferClobber: std::ffi::c_uint = 0;
const GLX_BufferSwapComplete: std::ffi::c_uint = 1;

const __GLX_NUMBER_EVENTS: std::ffi::c_uint = 17;

unsafe extern "system" {

    pub unsafe fn glXChooseVisual(
        dpy: *mut x11::xlib::Display,
        screen: std::ffi::c_int,
        attribList: *mut std::ffi::c_int,
    ) -> *mut XVisualInfo;

    pub unsafe fn glXCreateContext(
        dpy: *mut x11::xlib::Display,
        vis: *mut XVisualInfo,
        shareList: GLXContext,
        direct: x11::xlib::Bool,
    ) -> GLXContext;

    pub unsafe fn glXDestroyContext(dpy: *mut x11::xlib::Display, ctx: GLXContext);

    pub unsafe fn glXMakeCurrent(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
        ctx: GLXContext,
    ) -> x11::xlib::Bool;

    pub unsafe fn glXCopyContext(
        dpy: *mut x11::xlib::Display,
        src: GLXContext,
        dst: GLXContext,
        mask: std::ffi::c_ulong,
    );

    pub unsafe fn glXSwapBuffers(dpy: *mut x11::xlib::Display, drawable: GLXDrawable);

    pub unsafe fn glXCreateGLXPixmap(
        dpy: *mut x11::xlib::Display,
        visual: *mut XVisualInfo,
        pixmap: Pixmap,
    ) -> GLXPixmap;

    pub unsafe fn glXDestroyGLXPixmap(dpy: *mut x11::xlib::Display, GLXpixmap: Pixmap);

    pub unsafe fn glXQueryExtension(
        dpy: *mut x11::xlib::Display,
        errorb: *mut std::ffi::c_int,
        event: *mut std::ffi::c_int,
    ) -> x11::xlib::Bool;

    pub unsafe fn glXQueryVersion(
        dpy: *mut x11::xlib::Display,
        maj: *mut std::ffi::c_int,
        min: *mut std::ffi::c_int,
    ) -> x11::xlib::Bool;

    pub unsafe fn glXIsDirect(dpy: *mut x11::xlib::Display, ctx: GLXContext) -> x11::xlib::Bool;

    pub unsafe fn glXGetConfig(
        dpy: *mut x11::xlib::Display,
        visual: *mut XVisualInfo,
        attrib: std::ffi::c_int,
        value: *mut std::ffi::c_int,
    ) -> std::ffi::c_int;

    pub unsafe fn glXGetCurrentContext() -> x11::glx::GLXContext;

    pub unsafe fn glXGetCurrentDrawable() -> x11::glx::GLXContext;

    pub unsafe fn glXWaitGL();

    pub unsafe fn glXWaitX();

    pub unsafe fn glXUseXFont(
        font: x11::xlib::Font,
        first: std::ffi::c_int,
        count: std::ffi::c_int,
        list: std::ffi::c_int,
    );

    /* GLX 1.1 and later */
    pub unsafe fn glXQueryExtensionsString(
        dpy: *mut x11::xlib::Display,
        screen: std::ffi::c_int,
    ) -> *const std::ffi::c_char;

    pub unsafe fn glXQueryServerString(
        dpy: *mut x11::xlib::Display,
        screen: std::ffi::c_int,
        name: std::ffi::c_int,
    ) -> *const std::ffi::c_char;

    pub unsafe fn glXGetClientString(dpy: *mut x11::xlib::Display, name: std::ffi::c_int);

    /* GLX 1.2 and later */
    pub unsafe fn glXGetCurrentDisplay() -> *mut x11::xlib::Display;

    /* GLX 1.3 and later */
    pub unsafe fn glXChooseFBConfig(
        dpy: *mut x11::xlib::Display,
        screen: std::ffi::c_int,
        attribList: *const std::ffi::c_int,
        nitems: *mut std::ffi::c_int,
    ) -> *mut GLXFBConfig;

    pub unsafe fn glXGetFBConfigAttrib(
        dpy: *mut x11::xlib::Display,
        config: GLXFBConfig,
        attribute: std::ffi::c_int,
        value: *mut std::ffi::c_int,
    ) -> std::ffi::c_int;

    pub unsafe fn glXGetFBConfigs(
        dpy: *mut x11::xlib::Display,
        screen: std::ffi::c_int,
        nelements: *mut std::ffi::c_int,
    ) -> *mut GLXFBConfig;

    pub unsafe fn glXGetVisualFromFBConfig(
        dpy: *mut x11::xlib::Display,
        config: GLXFBConfig,
    ) -> *mut XVisualInfo;

    pub unsafe fn glXCreateWindow(
        dpy: *mut x11::xlib::Display,
        config: GLXFBConfig,
        win: x11::xlib::Window,
        attribList: *const std::ffi::c_int,
    ) -> GLXWindow;

    pub unsafe fn glXDestroyWindow(dpy: *mut x11::xlib::Display, window: GLXWindow);

    pub unsafe fn glXCreatePixmap(
        dpy: *mut x11::xlib::Display,
        config: GLXFBConfig,
        pixmap: Pixmap,
        attribList: *const std::ffi::c_int,
    ) -> GLXPixmap;

    pub unsafe fn glXDestroyPixmap(dpy: *mut x11::xlib::Display, pixmap: GLXPixmap);

    pub unsafe fn glXCreatePbuffer(
        dpy: *mut x11::xlib::Display,
        config: GLXFBConfig,
        attribList: *const std::ffi::c_int,
    ) -> GLXPbuffer;

    pub unsafe fn glXDestroyPbuffer(dpy: *mut x11::xlib::Display, pbuf: GLXPbuffer);

    pub unsafe fn glXQueryDrawable(
        dpy: *mut x11::xlib::Display,
        draw: GLXDrawable,
        attribute: std::ffi::c_int,
        value: *mut std::ffi::c_uint,
    );

    pub unsafe fn glXCreateNewContext(
        dpy: *mut x11::xlib::Display,
        config: GLXFBConfig,
        renderType: std::ffi::c_int,
        shareList: GLXContext,
        direct: x11::xlib::Bool,
    ) -> GLXContext;

    pub unsafe fn glXMakeContextCurrent(
        dpy: *mut x11::xlib::Display,
        draw: GLXDrawable,
        read: GLXDrawable,
        ctx: GLXContext,
    ) -> Bool;

    pub unsafe fn glXGetCurrentReadDrawable() -> GLXDrawable;

    pub unsafe fn glXQueryContext(
        dpy: *mut x11::xlib::Display,
        ctx: GLXContext,
        attribute: std::ffi::c_int,
        value: *mut std::ffi::c_int,
    ) -> std::ffi::c_int;

    pub unsafe fn glXSelectEvent(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
        mask: std::ffi::c_ulong,
    );

    pub unsafe fn glXGetSelectedEvent(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
        mask: *mut std::ffi::c_ulong,
    );
}

/* GLX 1.3 function pointer typedefs */

// typedef GLXFBConfig * (* PFNGLXGETFBCONFIGSPROC) (dpy: *mut x11::xlib::Display, screen:  std::ffi::c_int, nelements: *mut  std::ffi::c_int);
type PFNGLXGETFBCONFIGSPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    std::ffi::c_int,
    *mut std::ffi::c_int,
) -> *mut GLXFBConfig;
// typedef GLXFBConfig * (* PFNGLXCHOOSEFBCONFIGPROC) (dpy: *mut Display, screen:  std::ffi::c_int, attrib_list: *const std::ffi::c_int, nelements: *mut  std::ffi::c_int);
type PFNGLXCHOOSEFBCONFIGPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    std::ffi::c_int,
    *const std::ffi::c_int,
    *mut std::ffi::c_int,
) -> *mut GLXFBConfig;

// typedef int (* PFNGLXGETFBCONFIGATTRIBPROC) (dpy: *mut Display, config: GLXFBConfig, attribute:  std::ffi::c_int, value: *mut  std::ffi::c_int);
type PFNGLXGETFBCONFIGATTRIBPROC = unsafe extern "system" fn(
    *mut Display,
    GLXFBConfig,
    std::ffi::c_int,
    *mut std::ffi::c_int,
) -> std::ffi::c_int;
// typedef XVisualInfo * (* PFNGLXGETVISUALFROMFBCONFIGPROC) (dpy: *mut Display, GLXFBConfig config);
type PFNGLXGETVISUALFROMFBCONFIGPROC =
    unsafe extern "system" fn(*mut Display, GLXFBConfig) -> *mut XVisualInfo;
// typedef GLXWindow (* PFNGLXCREATEWINDOWPROC) (dpy: *mut Display, config: GLXFBConfig, win: Window, attrib_list: *const  std::ffi::c_int);
type PFNGLXCREATEWINDOWPROC =
    unsafe extern "system" fn(*mut Display, GLXFBConfig, x11::xlib::Window, *const std::ffi::c_int);
// typedef void (* PFNGLXDESTROYWINDOWPROC) (dpy: *mut Display, GLXWindow win);
type PFNGLXDESTROYWINDOWPROC = unsafe extern "system" fn(*mut x11::xlib::Display, GLXWindow) -> ();
// typedef GLXPixmap (* PFNGLXCREATEPIXMAPPROC) (dpy: *mut Display, config: GLXFBConfig, pixmap: Pixmap, attrib_list: *const  std::ffi::c_int);
type PFNGLXCREATEPIXMAPPROC =
    unsafe extern "system" fn(*mut x11::xlib::Display, GLXFBConfig, Pixmap, *const std::ffi::c_int);
// typedef void (* PFNGLXDESTROYPIXMAPPROC) (dpy: *mut Display, pixmap: GLXPixmap,);
type PFNGLXDESTROYPIXMAPPROC = unsafe extern "system" fn(*mut x11::xlib::Display, GLXPixmap);
// typedef GLXPbuffer (* PFNGLXCREATEPBUFFERPROC) (dpy: *mut Display, config: GLXFBConfig, attrib_list: *const  std::ffi::c_int);
type PFNGLXCREATEPBUFFERPROC =
    unsafe extern "system" fn(*mut x11::xlib::Display, GLXFBConfig, *const std::ffi::c_int);
// typedef void (* PFNGLXDESTROYPBUFFERPROC) (dpy: *mut Display, pbuf: GLXPbuffer,);
type PFNGLXDESTROYPBUFFERPROC = unsafe extern "system" fn(*mut x11::xlib::Display, GLXPbuffer);
// typedef void (* PFNGLXQUERYDRAWABLEPROC) (dpy: *mut Display, draw: GLXDrawable, attribute:  std::ffi::c_int, unsigned value: *mut  std::ffi::c_int);
type PFNGLXQUERYDRAWABLEPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    GLXDrawable,
    std::ffi::c_int,
    *mut std::ffi::c_uint,
) -> ();
// typedef GLXContext (* PFNGLXCREATENEWCONTEXTPROC) (dpy: *mut Display, config: GLXFBConfig, render_type:  std::ffi::c_int, share_list: GLXContext, direct:  x11::xlib::Bool);
type PFNGLXCREATENEWCONTEXTPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    GLXFBConfig,
    std::ffi::c_int,
    GLXContext,
    x11::xlib::Bool,
) -> GLXContext;
// typedef Bool (* PFNGLXMAKECONTEXTCURRENTPROC) (dpy: *mut Display, draw: GLXDrawable, read: GLXDrawable, ctx: GLXContext,);
type PFNGLXMAKECONTEXTCURRENTPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    GLXDrawable,
    GLXDrawable,
    GLXContext,
) -> x11::xlib::Bool;
// typedef GLXDrawable (* PFNGLXGETCURRENTREADDRAWABLEPROC) (void);
type PFNGLXGETCURRENTREADDRAWABLEPROC = unsafe extern "system" fn() -> GLXDrawable;
// typedef Display * (* PFNGLXGETCURRENTDISPLAYPROC) (void);
type PFNGLXGETCURRENTDISPLAYPROC = unsafe extern "system" fn() -> *mut x11::xlib::Display;
// typedef int (* PFNGLXQUERYCONTEXTPROC) (dpy: *mut x11::xlib::Display, ctx: GLXContext, attribute:  std::ffi::c_int, value: *mut  std::ffi::c_int);
type PFNGLXQUERYCONTEXTPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    GLXContext,
    std::ffi::c_int,
    *mut std::ffi::c_int,
) -> std::ffi::c_int;
// typedef void (* PFNGLXSELECTEVENTPROC) (dpy: *mut x11::xlib::Display, draw: GLXDrawable, event_mask:  std::ffi::c_ulong);
type PFNGLXSELECTEVENTPROC =
    unsafe extern "system" fn(*mut x11::xlib::Display, GLXDrawable, std::ffi::c_ulong);
// typedef void (* PFNGLXGETSELECTEDEVENTPROC) (dpy: *mut x11::xlib::Display, draw: GLXDrawable, unsigned long *event_mask);
type PFNGLXGETSELECTEDEVENTPROC =
    unsafe extern "system" fn(*mut x11::xlib::Display, GLXDrawable, std::ffi::c_ulong);

/*
 * ARB 2. GLX_ARB_get_proc_address
 */
// #ifndef GLX_ARB_get_proc_address
// #define GLX_ARB_get_proc_address 1

type __GLXextFuncPtr = unsafe extern "system" fn() -> ();

unsafe extern "system" {
    pub unsafe fn glxGetProcAddressARB(
        procedure_name: super::ConstGLubytePointer,
    ) -> __GLXextFuncPtr;
}

/* GLX 1.4 and later */

// is this an extension or a dynamically loaded function or what?

type glXGetProcAddress = unsafe extern "system" fn(super::ConstGLubytePointer);

unsafe extern "system" {
    pub unsafe fn glXGetProcAddress(procname: super::ConstGLubytePointer);
}

// extern void (*glXGetProcAddress(const GLubyte *procname))( void );

/* GLX 1.4 function pointer typedefs */
// is this correct?
// typedef __GLXextFuncPtr (* PFNGLXGETPROCADDRESSPROC) (const GLubyte *procName);
type PFNGLXGETPROCADDRESSPROC =
    unsafe extern "system" fn(super::ConstGLubytePointer) -> self::__GLXextFuncPtr;

// how to handle glxext?

// #ifndef GLX_GLXEXT_LEGACY

// #include <GL/glxext.h>

// #endif /* GLX_GLXEXT_LEGACY */

/**
 ** The following aren't in glxext.h yet.
 **/

/*
 * ???. GLX_NV_vertex_array_range
 */
// #ifndef GLX_NV_vertex_array_range
// #define GLX_NV_vertex_array_range

unsafe extern "system" {
    // extern void *glXAllocateMemoryNV(size:GLsizei, GLfloat readfreq, GLfloat writefreq, GLfloat priority);
    pub unsafe fn glXAllocateMemoryNV(
        size: super::GLsizei,
        readfreq: super::GLfloat,
        writefreq: super::GLfloat,
    ) -> ();

    // extern void glXFreeMemoryNV(*pointer:GLvoid);
    pub unsafe fn glXFreeMemoryNV(pointer: *mut super::GLvoid) -> ();
}

// typedef void * ( * PFNGLXALLOCATEMEMORYNVPROC) (size:GLsizei, GLfloat readfreq, GLfloat writefreq, GLfloat priority);
type PFNGLXALLOCATEMEMORYNVPROC =
    unsafe extern "system" fn(super::GLsizei, super::GLfloat, super::GLfloat, super::GLfloat) -> ();
// typedef void ( * PFNGLXFREEMEMORYNVPROC) (*pointer:GLvoid);
type PFNGLXFREEMEMORYNVPROC = unsafe extern "system" fn(super::GLvoid) -> ();

// #endif /* GLX_NV_vertex_array_range */

/*
 * ARB ?. GLX_ARB_render_texture
 * XXX This was never finalized!
 */
// #ifndef GLX_ARB_render_texture
// #define GLX_ARB_render_texture 1

unsafe extern "system" {
    pub unsafe fn glXBindTexImageARB(
        dpy: *mut x11::xlib::Display,
        pbuffer: GLXPbuffer,
        buffer: std::ffi::c_int,
    ) -> x11::xlib::Bool;
    pub unsafe fn glXReleaseTexImageARB(
        dpy: *mut x11::xlib::Display,
        pbuffer: GLXPbuffer,
        buffer: std::ffi::c_int,
    ) -> x11::xlib::Bool;
    pub unsafe fn glXDrawableAttribARB(
        dpy: *mut x11::xlib::Display,
        draw: GLXDrawable,
        attribList: *const std::ffi::c_int,
    ) -> x11::xlib::Bool;
}

// #endif /* GLX_ARB_render_texture */

/*
 * Remove this when glxext.h is updated.
 */
// #ifndef GLX_NV_float_buffer
// const GLX_NV_float_buffer: std::ffi::c_uint = 1;

const GLX_FLOAT_COMPONENTS_NV: std::ffi::c_uint = 0x20B0;

// #endif /* GLX_NV_float_buffer */

/*
 * #?. GLX_MESA_swap_frame_usage
 */
// #ifndef GLX_MESA_swap_frame_usage
// #define GLX_MESA_swap_frame_usage 1
unsafe extern "system" {
    pub unsafe fn glXGetFrameUsageMESA(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
        usage: *mut std::ffi::c_float,
    ) -> std::ffi::c_int;
    pub unsafe fn glXBeginFrameTrackingMESA(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
    ) -> std::ffi::c_int;
    pub unsafe fn glXEndFrameTrackingMESA(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
    ) -> std::ffi::c_int;
    pub unsafe fn glXQueryFrameTrackingMESA(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
        swapCount: *mut i64,
        missedFrames: *mut i64,
        lastmissedusage: *mut std::ffi::c_float,
    ) -> std::ffi::c_int;

}

// typedef int (*PFNGLXGETFRAMEUSAGEMESAPROC) (dpy: *mut x11::xlib::Display, drawable: GLXDrawable, float *usage);
type PFNGLXGETFRAMEUSAGEMESAPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    GLXDrawable,
    *mut std::ffi::c_float,
) -> std::ffi::c_int;
// typedef int (*PFNGLXBEGINFRAMETRACKINGMESAPROC)(dpy: *mut x11::xlib::Display, drawable: GLXDrawable);
type PFNGLXBEGINFRAMETRACKINGMESAPROC =
    unsafe extern "system" fn(*mut x11::xlib::Display, GLXDrawable) -> std::ffi::c_int;
// typedef int (*PFNGLXENDFRAMETRACKINGMESAPROC)(dpy: *mut x11::xlib::Display, drawable: GLXDrawable);
type PFNGLXENDFRAMETRACKINGMESAPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    GLXDrawable,
    *mut std::ffi::c_float,
) -> std::ffi::c_int;
// typedef int (*PFNGLXQUERYFRAMETRACKINGMESAPROC)(dpy: *mut x11::xlib::Display, drawable: GLXDrawable, int64_t *swapCount, int64_t *missedFrames, float *lastMissedUsage);
type PFNGLXQUERYFRAMETRACKINGMESAPROC = unsafe extern "system" fn(
    *mut x11::xlib::Display,
    GLXDrawable,
    *mut i64,
    *mut i64,
    *mut std::ffi::c_float,
) -> std::ffi::c_int;

// #endif /* GLX_MESA_swap_frame_usage */

/*
 * #?. GLX_MESA_swap_control
 */
// #ifndef GLX_MESA_swap_control
// #define GLX_MESA_swap_control 1

unsafe extern "system" {

    pub unsafe fn glXSwapIntervalMESA(interval: std::ffi::c_uint) -> std::ffi::c_int;
    pub unsafe fn glXGetSwapIntervalMESA() -> std::ffi::c_int;
}

// typedef int (*PFNGLXSWAPINTERVALMESAPROC)(unsigned interval:  std::ffi::c_int);
type PFNGLXSWAPINTERVALMESAPROC = unsafe extern "system" fn(std::ffi::c_uint);
// typedef int PFNGLXGETSWAPINTERVALMESAPROC)(void);
type PFNGLXGETSWAPINTERVALMESAPROC = unsafe extern "system" fn() -> std::ffi::c_uint;

// #endif /* GLX_MESA_swap_control */

/*
 * #?. GLX_EXT_texture_from_pixmap
 * XXX not finished?
 */
// #ifndef GLX_EXT_texture_from_pixmap
// #define GLX_EXT_texture_from_pixmap 1

const GLX_BIND_TO_TEXTURE_RGB_EXT: std::ffi::c_uint = 0x20D0;
const GLX_BIND_TO_TEXTURE_RGBA_EXT: std::ffi::c_uint = 0x20D1;
const GLX_BIND_TO_MIPMAP_TEXTURE_EXT: std::ffi::c_uint = 0x20D2;
const GLX_BIND_TO_TEXTURE_TARGETS_EXT: std::ffi::c_uint = 0x20D3;
const GLX_Y_INVERTED_EXT: std::ffi::c_uint = 0x20D4;

const GLX_TEXTURE_FORMAT_EXT: std::ffi::c_uint = 0x20D5;
const GLX_TEXTURE_TARGET_EXT: std::ffi::c_uint = 0x20D6;
const GLX_MIPMAP_TEXTURE_EXT: std::ffi::c_uint = 0x20D7;

const GLX_TEXTURE_FORMAT_NONE_EXT: std::ffi::c_uint = 0x20D8;
const GLX_TEXTURE_FORMAT_RGB_EXT: std::ffi::c_uint = 0x20D9;
const GLX_TEXTURE_FORMAT_RGBA_EXT: std::ffi::c_uint = 0x20DA;

const GLX_TEXTURE_1D_BIT_EXT: std::ffi::c_uint = 0x00000001;
const GLX_TEXTURE_2D_BIT_EXT: std::ffi::c_uint = 0x00000002;
const GLX_TEXTURE_RECTANGLE_BIT_EXT: std::ffi::c_uint = 0x00000004;

const GLX_TEXTURE_1D_EXT: std::ffi::c_uint = 0x20DB;
const GLX_TEXTURE_2D_EXT: std::ffi::c_uint = 0x20DC;
const GLX_TEXTURE_RECTANGLE_EXT: std::ffi::c_uint = 0x20DD;

const GLX_FRONT_LEFT_EXT: std::ffi::c_uint = 0x20DE;
const GLX_FRONT_RIGHT_EXT: std::ffi::c_uint = 0x20DF;
const GLX_BACK_LEFT_EXT: std::ffi::c_uint = 0x20E0;
const GLX_BACK_RIGHT_EXT: std::ffi::c_uint = 0x20E1;
// #define GLX_FRONT_EXT                      GLX_FRONT_LEFT_EXT
// #define GLX_BACK_EXT                       GLX_BACK_LEFT_EXT
const GLX_AUX0_EXT: std::ffi::c_uint = 0x20E2;
const GLX_AUX1_EXT: std::ffi::c_uint = 0x20E3;
const GLX_AUX2_EXT: std::ffi::c_uint = 0x20E4;
const GLX_AUX3_EXT: std::ffi::c_uint = 0x20E5;
const GLX_AUX4_EXT: std::ffi::c_uint = 0x20E6;
const GLX_AUX5_EXT: std::ffi::c_uint = 0x20E7;
const GLX_AUX6_EXT: std::ffi::c_uint = 0x20E8;
const GLX_AUX7_EXT: std::ffi::c_uint = 0x20E9;
const GLX_AUX8_EXT: std::ffi::c_uint = 0x20EA;
const GLX_AUX9_EXT: std::ffi::c_uint = 0x20EB;

unsafe extern "C" {
    pub unsafe fn glXBindTexImageEXT(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
        buffer: std::ffi::c_int,
        attrib_list: *const std::ffi::c_int,
    );
    // pub unsafe fn glXBindTexImageEXT(dpy *mut )
    pub unsafe fn glXReleaseTexImageEXT(
        dpy: *mut x11::xlib::Display,
        drawable: GLXDrawable,
        buffer: std::ffi::c_int,
    );

}

// #endif /* GLX_EXT_texture_from_pixmap */

/*** Should these go here, or in another header? */
/*
** GLX Events
*/
#[repr(C)]
struct GLXPbufferClobberEvent {
    event_type: std::ffi::c_int,      /* GLX_DAMAGED or GLX_SAVED */
    draw_type: std::ffi::c_int,       /* GLX_WINDOW or GLX_PBUFFER */
    serial: std::ffi::c_ulong,        /* # of last request processed by server */
    send_event: x11::xlib::Bool,      /* true if this came for SendEvent request */
    display: *mut x11::xlib::Display, /* display the event was read from */
    drawable: GLXDrawable,            /* XID of Drawable */
    buffer_mask: std::ffi::c_uint,    /* mask indicating which buffers are affected */
    aux_buffer: std::ffi::c_uint,     /* which aux buffer was affected */
    x: std::ffi::c_uint,
    y: std::ffi::c_uint,
    width: std::ffi::c_int,
    height: std::ffi::c_int,
    /* if nonzero, at least this many more */
    count: std::ffi::c_int,
}
#[repr(C)]
struct GLXBufferSwapComplete {
    r#type: std::ffi::c_int,
    serial: std::ffi::c_ulong,   /* # of last request processed by server */
    send_event: x11::xlib::Bool, /* true if this came from a SendEvent request */
    display: *mut x11::xlib::Display, /* Display the event was read from */
    drawable: GLXDrawable,       /* drawable on which event was requested in event mask */
    event_type: std::ffi::c_int,
    ust: i64,
    msc: i64,
    sbc: i64,
}
#[repr(C)]
union _GLXEvent {
    glxpbufferclobber: std::mem::ManuallyDrop<GLXPbufferClobberEvent>,
    glxbufferswapcomplete: std::mem::ManuallyDrop<GLXBufferSwapComplete>,
    pad: [std::ffi::c_ulong; 24],
}
