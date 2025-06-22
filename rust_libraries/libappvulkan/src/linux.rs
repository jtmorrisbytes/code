// compile_error!("TODO: Move any X11 / XORG typedefs into rust_libraries/libxorg");
use libxorg::bindings::{Display, VisualID, Window, XErrorEvent, XPointer,_XIMStatusDrawCallbackStruct,KeySym};

use crate::bindings::VkPhysicalDevice;
use crate::bindings::{
    __dev_t, __fsid_t, __gid_t, __ino_t, __int_least16_t, __int_least32_t, __int_least64_t,
    __int_least8_t, __intmax_t, __mode_t, __nlink_t, __off64_t, __off_t, __pid_t, __quad_t,
    __u_char, __u_int, __u_long, __u_quad_t, __u_short, __uid_t, __uint16_t, __uint32_t,
    __uint64_t, __uint8_t, __uint_least16_t, __uint_least32_t, __uint_least64_t, __uint_least8_t,
    __uintmax_t,
};
use crate::bindings::{
    wchar_t, VkAllocationCallbacks, VkBool32, VkDisplayKHR, VkFlags, VkResult, VkStructureType,
    VkSurfaceKHR,
};
use crate::bindings::VkInstance;

pub const VK_KHR_xlib_surface: u32 = 1;
pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &::std::ffi::CStr = c"VK_KHR_xlib_surface";

pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &::std::ffi::CStr = c"VK_KHR_wayland_surface";

pub const VULKAN_XLIB_H_: u32 = 1;


const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = c"VK_EXT_acquire_xlib_display";

type PFN_vkAcquireXlibDisplayEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    display: VkDisplayKHR,
) -> VkResult;
// type PFN_vkGetRandROutputDisplayEXT = unsafe extern "C" fn(
//     physicalDevice: VkPhysicalDevice,
//     dpy: *mut Display,
//     rrOutput: RROutput,
//     pDisplay: *mut VkDisplayKHR,
// ) -> VkResult;


// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of _XIMStatusDrawCallbackStruct__bindgen_ty_1"]
//         [::std::mem::size_of::<_XIMStatusDrawCallbackStruct__bindgen_ty_1>() - 8usize];
//     ["Alignment of _XIMStatusDrawCallbackStruct__bindgen_ty_1"]
//         [::std::mem::align_of::<_XIMStatusDrawCallbackStruct__bindgen_ty_1>() - 8usize];
//     ["Offset of field: _XIMStatusDrawCallbackStruct__bindgen_ty_1::text"]
//         [::std::mem::offset_of!(_XIMStatusDrawCallbackStruct__bindgen_ty_1, text) - 0usize];
//     ["Offset of field: _XIMStatusDrawCallbackStruct__bindgen_ty_1::bitmap"]
//         [::std::mem::offset_of!(_XIMStatusDrawCallbackStruct__bindgen_ty_1, bitmap) - 0usize];
// };
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of _XIMStatusDrawCallbackStruct"]
//         [::std::mem::size_of::<_XIMStatusDrawCallbackStruct>() - 16usize];
//     ["Alignment of _XIMStatusDrawCallbackStruct"]
//         [::std::mem::align_of::<_XIMStatusDrawCallbackStruct>() - 8usize];
//     ["Offset of field: _XIMStatusDrawCallbackStruct::type_"]
//         [::std::mem::offset_of!(_XIMStatusDrawCallbackStruct, type_) - 0usize];
//     ["Offset of field: _XIMStatusDrawCallbackStruct::data"]
//         [::std::mem::offset_of!(_XIMStatusDrawCallbackStruct, data) - 8usize];
// };
#[cfg(target_os = "linux")]
pub type XIMStatusDrawCallbackStruct = _XIMStatusDrawCallbackStruct;
#[repr(C)]
#[cfg(target_os = "linux")]
#[derive(Debug, Copy, Clone)]
pub struct _XIMHotKeyTrigger {
    pub keysym: KeySym,
    pub modifier: ::std::os::raw::c_int,
    pub modifier_mask: ::std::os::raw::c_int,
}
// #[cfg(target_os = "linux")]
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of _XIMHotKeyTrigger"][::std::mem::size_of::<_XIMHotKeyTrigger>() - 16usize];
//     ["Alignment of _XIMHotKeyTrigger"][::std::mem::align_of::<_XIMHotKeyTrigger>() - 8usize];
//     ["Offset of field: _XIMHotKeyTrigger::keysym"]
//         [::std::mem::offset_of!(_XIMHotKeyTrigger, keysym) - 0usize];
//     ["Offset of field: _XIMHotKeyTrigger::modifier"]
//         [::std::mem::offset_of!(_XIMHotKeyTrigger, modifier) - 8usize];
//     ["Offset of field: _XIMHotKeyTrigger::modifier_mask"]
//         [::std::mem::offset_of!(_XIMHotKeyTrigger, modifier_mask) - 12usize];
// };

pub type XIMHotKeyTrigger = _XIMHotKeyTrigger;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIMHotKeyTriggers {
    pub num_hot_key: ::std::os::raw::c_int,
    pub key: *mut XIMHotKeyTrigger,
}
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of _XIMHotKeyTriggers"][::std::mem::size_of::<_XIMHotKeyTriggers>() - 16usize];
//     ["Alignment of _XIMHotKeyTriggers"][::std::mem::align_of::<_XIMHotKeyTriggers>() - 8usize];
//     ["Offset of field: _XIMHotKeyTriggers::num_hot_key"]
//         [::std::mem::offset_of!(_XIMHotKeyTriggers, num_hot_key) - 0usize];
//     ["Offset of field: _XIMHotKeyTriggers::key"]
//         [::std::mem::offset_of!(_XIMHotKeyTriggers, key) - 8usize];
// };

pub type XIMHotKeyTriggers = _XIMHotKeyTriggers;
pub type XIMHotKeyState = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMValuesList {
    pub count_values: ::std::os::raw::c_ushort,
    pub supported_values: *mut *mut ::std::os::raw::c_char,
}
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of XIMValuesList"][::std::mem::size_of::<XIMValuesList>() - 16usize];
//     ["Alignment of XIMValuesList"][::std::mem::align_of::<XIMValuesList>() - 8usize];
//     ["Offset of field: XIMValuesList::count_values"]
//         [::std::mem::offset_of!(XIMValuesList, count_values) - 0usize];
//     ["Offset of field: XIMValuesList::supported_values"]
//         [::std::mem::offset_of!(XIMValuesList, supported_values) - 8usize];
// };
pub type XErrorHandler = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Display, arg2: *mut XErrorEvent) -> ::std::os::raw::c_int,
>;
pub type XIOErrorHandler =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut Display) -> ::std::os::raw::c_int>;
pub type XIOErrorExitHandler = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Display, arg2: *mut ::std::os::raw::c_void),
>;
pub type XConnectionWatchProc = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut Display,
        arg2: XPointer,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: *mut XPointer,
    ),
>;
pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const ::std::os::raw::c_void,
    pub flags: VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}
// #[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of VkXlibSurfaceCreateInfoKHR"]
//         [::std::mem::size_of::<VkXlibSurfaceCreateInfoKHR>() - 40usize];
//     ["Alignment of VkXlibSurfaceCreateInfoKHR"]
//         [::std::mem::align_of::<VkXlibSurfaceCreateInfoKHR>() - 8usize];
//     ["Offset of field: VkXlibSurfaceCreateInfoKHR::s_type"]
//         [::std::mem::offset_of!(VkXlibSurfaceCreateInfoKHR, s_type) - 0usize];
//     ["Offset of field: VkXlibSurfaceCreateInfoKHR::p_next"]
//         [::std::mem::offset_of!(VkXlibSurfaceCreateInfoKHR, p_next) - 8usize];
//     ["Offset of field: VkXlibSurfaceCreateInfoKHR::flags"]
//         [::std::mem::offset_of!(VkXlibSurfaceCreateInfoKHR, flags) - 16usize];
//     ["Offset of field: VkXlibSurfaceCreateInfoKHR::dpy"]
//         [::std::mem::offset_of!(VkXlibSurfaceCreateInfoKHR, dpy) - 24usize];
//     ["Offset of field: VkXlibSurfaceCreateInfoKHR::window"]
//         [::std::mem::offset_of!(VkXlibSurfaceCreateInfoKHR, window) - 32usize];
// };
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXlibSurfaceKHR = ::std::option::Option<
    unsafe extern "C" fn(
        instance: VkInstance,
        pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
>;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = ::std::option::Option<
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        dpy: *mut Display,
        visualID: VisualID,
    ) -> VkBool32,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WlDisplay {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WlSurface {
    pub _address: u8,
}

#[cfg(any(target_os = "linux"))]
extern "C" {
    pub fn vkCreateXlibSurfaceKHR(
        instance: crate::bindings::VkInstance,
        pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        dpy: *mut Display,
        visualID: VisualID,
    ) -> VkBool32;

    pub fn vkCreateWaylandSurfaceKHR(
        instance: crate::bindings::VkInstance,
        pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        display: *mut WlDisplay,
    ) -> VkBool32;
}

pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const ::std::os::raw::c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut WlDisplay,
    pub surface: *mut WlSurface,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VkWaylandSurfaceCreateInfoKHR"]
        [::std::mem::size_of::<VkWaylandSurfaceCreateInfoKHR>() - 40usize];
    ["Alignment of VkWaylandSurfaceCreateInfoKHR"]
        [::std::mem::align_of::<VkWaylandSurfaceCreateInfoKHR>() - 8usize];
    ["Offset of field: VkWaylandSurfaceCreateInfoKHR::s_type"]
        [::std::mem::offset_of!(VkWaylandSurfaceCreateInfoKHR, s_type) - 0usize];
    ["Offset of field: VkWaylandSurfaceCreateInfoKHR::p_next"]
        [::std::mem::offset_of!(VkWaylandSurfaceCreateInfoKHR, p_next) - 8usize];
    ["Offset of field: VkWaylandSurfaceCreateInfoKHR::flags"]
        [::std::mem::offset_of!(VkWaylandSurfaceCreateInfoKHR, flags) - 16usize];
    ["Offset of field: VkWaylandSurfaceCreateInfoKHR::display"]
        [::std::mem::offset_of!(VkWaylandSurfaceCreateInfoKHR, display) - 24usize];
    ["Offset of field: VkWaylandSurfaceCreateInfoKHR::surface"]
        [::std::mem::offset_of!(VkWaylandSurfaceCreateInfoKHR, surface) - 32usize];
};
pub type PFN_vkCreateWaylandSurfaceKHR = ::std::option::Option<
    unsafe extern "C" fn(
        instance: crate::bindings::VkInstance,
        pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
>;
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = ::std::option::Option<
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        display: *mut WlDisplay,
    ) -> VkBool32,
>;

impl crate::Loader {
    pub fn vk_create_wayland_surface_khr(
        instance: crate::bindings::VkInstance,
        p_create_info: *const VkWaylandSurfaceCreateInfoKHR,
        p_allocator: *const VkAllocationCallbacks,
        p_surface: *mut VkSurfaceKHR,
    ) -> VkResult {
        todo!()
    }
}