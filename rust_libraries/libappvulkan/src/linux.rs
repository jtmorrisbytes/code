compile_error!("TODO: Move any X11 / XORG typedefs into rust_libraries/libxorg");


#![allow(non_upper_case_globals, unused)]
#[cfg(any(target_os = "linux"))]
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

pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;

pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::std::os::raw::c_long;

const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = c"VK_EXT_acquire_xlib_display";

type PFN_vkAcquireXlibDisplayEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    display: VkDisplayKHR,
) -> VkResult;
type PFN_vkGetRandROutputDisplayEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    rrOutput: x11::xrandr::RROutput,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult;



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of timespec"][::std::mem::size_of::<timespec>() - 16usize];
    ["Alignment of timespec"][::std::mem::align_of::<timespec>() - 8usize];
    ["Offset of field: timespec::tv_sec"][::std::mem::offset_of!(timespec, tv_sec) - 0usize];
    ["Offset of field: timespec::tv_nsec"][::std::mem::offset_of!(timespec, tv_nsec) - 8usize];
};
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fd_set"][::std::mem::size_of::<fd_set>() - 128usize];
    ["Alignment of fd_set"][::std::mem::align_of::<fd_set>() - 8usize];
    ["Offset of field: fd_set::__fds_bits"][::std::mem::offset_of!(fd_set, __fds_bits) - 0usize];
};
pub type fd_mask = __fd_mask;
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __atomic_wide_counter {
    pub __value64: ::std::os::raw::c_ulonglong,
    pub __value32: __atomic_wide_counter__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __atomic_wide_counter__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __atomic_wide_counter__bindgen_ty_1"]
        [::std::mem::size_of::<__atomic_wide_counter__bindgen_ty_1>() - 8usize];
    ["Alignment of __atomic_wide_counter__bindgen_ty_1"]
        [::std::mem::align_of::<__atomic_wide_counter__bindgen_ty_1>() - 4usize];
    ["Offset of field: __atomic_wide_counter__bindgen_ty_1::__low"]
        [::std::mem::offset_of!(__atomic_wide_counter__bindgen_ty_1, __low) - 0usize];
    ["Offset of field: __atomic_wide_counter__bindgen_ty_1::__high"]
        [::std::mem::offset_of!(__atomic_wide_counter__bindgen_ty_1, __high) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __atomic_wide_counter"][::std::mem::size_of::<__atomic_wide_counter>() - 8usize];
    ["Alignment of __atomic_wide_counter"]
        [::std::mem::align_of::<__atomic_wide_counter>() - 8usize];
    ["Offset of field: __atomic_wide_counter::__value64"]
        [::std::mem::offset_of!(__atomic_wide_counter, __value64) - 0usize];
    ["Offset of field: __atomic_wide_counter::__value32"]
        [::std::mem::offset_of!(__atomic_wide_counter, __value32) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_internal_list"][::std::mem::size_of::<__pthread_internal_list>() - 16usize];
    ["Alignment of __pthread_internal_list"]
        [::std::mem::align_of::<__pthread_internal_list>() - 8usize];
    ["Offset of field: __pthread_internal_list::__prev"]
        [::std::mem::offset_of!(__pthread_internal_list, __prev) - 0usize];
    ["Offset of field: __pthread_internal_list::__next"]
        [::std::mem::offset_of!(__pthread_internal_list, __next) - 8usize];
};
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_internal_slist"]
        [::std::mem::size_of::<__pthread_internal_slist>() - 8usize];
    ["Alignment of __pthread_internal_slist"]
        [::std::mem::align_of::<__pthread_internal_slist>() - 8usize];
    ["Offset of field: __pthread_internal_slist::__next"]
        [::std::mem::offset_of!(__pthread_internal_slist, __next) - 0usize];
};
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_mutex_s"][::std::mem::size_of::<__pthread_mutex_s>() - 40usize];
    ["Alignment of __pthread_mutex_s"][::std::mem::align_of::<__pthread_mutex_s>() - 8usize];
    ["Offset of field: __pthread_mutex_s::__lock"]
        [::std::mem::offset_of!(__pthread_mutex_s, __lock) - 0usize];
    ["Offset of field: __pthread_mutex_s::__count"]
        [::std::mem::offset_of!(__pthread_mutex_s, __count) - 4usize];
    ["Offset of field: __pthread_mutex_s::__owner"]
        [::std::mem::offset_of!(__pthread_mutex_s, __owner) - 8usize];
    ["Offset of field: __pthread_mutex_s::__nusers"]
        [::std::mem::offset_of!(__pthread_mutex_s, __nusers) - 12usize];
    ["Offset of field: __pthread_mutex_s::__kind"]
        [::std::mem::offset_of!(__pthread_mutex_s, __kind) - 16usize];
    ["Offset of field: __pthread_mutex_s::__spins"]
        [::std::mem::offset_of!(__pthread_mutex_s, __spins) - 20usize];
    ["Offset of field: __pthread_mutex_s::__elision"]
        [::std::mem::offset_of!(__pthread_mutex_s, __elision) - 22usize];
    ["Offset of field: __pthread_mutex_s::__list"]
        [::std::mem::offset_of!(__pthread_mutex_s, __list) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_rwlock_arch_t"][::std::mem::size_of::<__pthread_rwlock_arch_t>() - 56usize];
    ["Alignment of __pthread_rwlock_arch_t"]
        [::std::mem::align_of::<__pthread_rwlock_arch_t>() - 8usize];
    ["Offset of field: __pthread_rwlock_arch_t::__readers"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __readers) - 0usize];
    ["Offset of field: __pthread_rwlock_arch_t::__writers"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __writers) - 4usize];
    ["Offset of field: __pthread_rwlock_arch_t::__wrphase_futex"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __wrphase_futex) - 8usize];
    ["Offset of field: __pthread_rwlock_arch_t::__writers_futex"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __writers_futex) - 12usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad3"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad3) - 16usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad4"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad4) - 20usize];
    ["Offset of field: __pthread_rwlock_arch_t::__cur_writer"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __cur_writer) - 24usize];
    ["Offset of field: __pthread_rwlock_arch_t::__shared"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __shared) - 28usize];
    ["Offset of field: __pthread_rwlock_arch_t::__rwelision"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __rwelision) - 32usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad1"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad1) - 33usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad2"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad2) - 40usize];
    ["Offset of field: __pthread_rwlock_arch_t::__flags"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __flags) - 48usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_cond_s"][::std::mem::size_of::<__pthread_cond_s>() - 48usize];
    ["Alignment of __pthread_cond_s"][::std::mem::align_of::<__pthread_cond_s>() - 8usize];
    ["Offset of field: __pthread_cond_s::__wseq"]
        [::std::mem::offset_of!(__pthread_cond_s, __wseq) - 0usize];
    ["Offset of field: __pthread_cond_s::__g1_start"]
        [::std::mem::offset_of!(__pthread_cond_s, __g1_start) - 8usize];
    ["Offset of field: __pthread_cond_s::__g_refs"]
        [::std::mem::offset_of!(__pthread_cond_s, __g_refs) - 16usize];
    ["Offset of field: __pthread_cond_s::__g_size"]
        [::std::mem::offset_of!(__pthread_cond_s, __g_size) - 24usize];
    ["Offset of field: __pthread_cond_s::__g1_orig_size"]
        [::std::mem::offset_of!(__pthread_cond_s, __g1_orig_size) - 32usize];
    ["Offset of field: __pthread_cond_s::__wrefs"]
        [::std::mem::offset_of!(__pthread_cond_s, __wrefs) - 36usize];
    ["Offset of field: __pthread_cond_s::__g_signals"]
        [::std::mem::offset_of!(__pthread_cond_s, __g_signals) - 40usize];
};
pub type __tss_t = ::std::os::raw::c_uint;
pub type __thrd_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __once_flag {
    pub __data: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __once_flag"][::std::mem::size_of::<__once_flag>() - 4usize];
    ["Alignment of __once_flag"][::std::mem::align_of::<__once_flag>() - 4usize];
    ["Offset of field: __once_flag::__data"][::std::mem::offset_of!(__once_flag, __data) - 0usize];
};
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_mutexattr_t"][::std::mem::size_of::<pthread_mutexattr_t>() - 4usize];
    ["Alignment of pthread_mutexattr_t"][::std::mem::align_of::<pthread_mutexattr_t>() - 4usize];
    ["Offset of field: pthread_mutexattr_t::__size"]
        [::std::mem::offset_of!(pthread_mutexattr_t, __size) - 0usize];
    ["Offset of field: pthread_mutexattr_t::__align"]
        [::std::mem::offset_of!(pthread_mutexattr_t, __align) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_condattr_t"][::std::mem::size_of::<pthread_condattr_t>() - 4usize];
    ["Alignment of pthread_condattr_t"][::std::mem::align_of::<pthread_condattr_t>() - 4usize];
    ["Offset of field: pthread_condattr_t::__size"]
        [::std::mem::offset_of!(pthread_condattr_t, __size) - 0usize];
    ["Offset of field: pthread_condattr_t::__align"]
        [::std::mem::offset_of!(pthread_condattr_t, __align) - 0usize];
};
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_attr_t"][::std::mem::size_of::<pthread_attr_t>() - 56usize];
    ["Alignment of pthread_attr_t"][::std::mem::align_of::<pthread_attr_t>() - 8usize];
    ["Offset of field: pthread_attr_t::__size"]
        [::std::mem::offset_of!(pthread_attr_t, __size) - 0usize];
    ["Offset of field: pthread_attr_t::__align"]
        [::std::mem::offset_of!(pthread_attr_t, __align) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_mutex_t"][::std::mem::size_of::<pthread_mutex_t>() - 40usize];
    ["Alignment of pthread_mutex_t"][::std::mem::align_of::<pthread_mutex_t>() - 8usize];
    ["Offset of field: pthread_mutex_t::__data"]
        [::std::mem::offset_of!(pthread_mutex_t, __data) - 0usize];
    ["Offset of field: pthread_mutex_t::__size"]
        [::std::mem::offset_of!(pthread_mutex_t, __size) - 0usize];
    ["Offset of field: pthread_mutex_t::__align"]
        [::std::mem::offset_of!(pthread_mutex_t, __align) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_cond_t"][::std::mem::size_of::<pthread_cond_t>() - 48usize];
    ["Alignment of pthread_cond_t"][::std::mem::align_of::<pthread_cond_t>() - 8usize];
    ["Offset of field: pthread_cond_t::__data"]
        [::std::mem::offset_of!(pthread_cond_t, __data) - 0usize];
    ["Offset of field: pthread_cond_t::__size"]
        [::std::mem::offset_of!(pthread_cond_t, __size) - 0usize];
    ["Offset of field: pthread_cond_t::__align"]
        [::std::mem::offset_of!(pthread_cond_t, __align) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_rwlock_t"][::std::mem::size_of::<pthread_rwlock_t>() - 56usize];
    ["Alignment of pthread_rwlock_t"][::std::mem::align_of::<pthread_rwlock_t>() - 8usize];
    ["Offset of field: pthread_rwlock_t::__data"]
        [::std::mem::offset_of!(pthread_rwlock_t, __data) - 0usize];
    ["Offset of field: pthread_rwlock_t::__size"]
        [::std::mem::offset_of!(pthread_rwlock_t, __size) - 0usize];
    ["Offset of field: pthread_rwlock_t::__align"]
        [::std::mem::offset_of!(pthread_rwlock_t, __align) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_rwlockattr_t"][::std::mem::size_of::<pthread_rwlockattr_t>() - 8usize];
    ["Alignment of pthread_rwlockattr_t"][::std::mem::align_of::<pthread_rwlockattr_t>() - 8usize];
    ["Offset of field: pthread_rwlockattr_t::__size"]
        [::std::mem::offset_of!(pthread_rwlockattr_t, __size) - 0usize];
    ["Offset of field: pthread_rwlockattr_t::__align"]
        [::std::mem::offset_of!(pthread_rwlockattr_t, __align) - 0usize];
};
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_barrier_t"][::std::mem::size_of::<pthread_barrier_t>() - 32usize];
    ["Alignment of pthread_barrier_t"][::std::mem::align_of::<pthread_barrier_t>() - 8usize];
    ["Offset of field: pthread_barrier_t::__size"]
        [::std::mem::offset_of!(pthread_barrier_t, __size) - 0usize];
    ["Offset of field: pthread_barrier_t::__align"]
        [::std::mem::offset_of!(pthread_barrier_t, __align) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_barrierattr_t"][::std::mem::size_of::<pthread_barrierattr_t>() - 4usize];
    ["Alignment of pthread_barrierattr_t"]
        [::std::mem::align_of::<pthread_barrierattr_t>() - 4usize];
    ["Offset of field: pthread_barrierattr_t::__size"]
        [::std::mem::offset_of!(pthread_barrierattr_t, __size) - 0usize];
    ["Offset of field: pthread_barrierattr_t::__align"]
        [::std::mem::offset_of!(pthread_barrierattr_t, __align) - 0usize];
};
pub type XID = ::std::os::raw::c_ulong;
pub type Mask = ::std::os::raw::c_ulong;
pub type Atom = ::std::os::raw::c_ulong;
pub type VisualID = ::std::os::raw::c_ulong;
pub type Time = ::std::os::raw::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type GContext = XID;
pub type KeySym = XID;
pub type KeyCode = ::std::os::raw::c_uchar;
pub type XPointer = *mut ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XExtData {
    pub number: ::std::os::raw::c_int,
    pub next: *mut _XExtData,
    pub free_private: ::std::option::Option<
        unsafe extern "C" fn(extension: *mut _XExtData) -> ::std::os::raw::c_int,
    >,
    pub private_data: XPointer,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XExtData"][::std::mem::size_of::<_XExtData>() - 32usize];
    ["Alignment of _XExtData"][::std::mem::align_of::<_XExtData>() - 8usize];
    ["Offset of field: _XExtData::number"][::std::mem::offset_of!(_XExtData, number) - 0usize];
    ["Offset of field: _XExtData::next"][::std::mem::offset_of!(_XExtData, next) - 8usize];
    ["Offset of field: _XExtData::free_private"]
        [::std::mem::offset_of!(_XExtData, free_private) - 16usize];
    ["Offset of field: _XExtData::private_data"]
        [::std::mem::offset_of!(_XExtData, private_data) - 24usize];
};
pub type XExtData = _XExtData;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XExtCodes {
    pub extension: ::std::os::raw::c_int,
    pub major_opcode: ::std::os::raw::c_int,
    pub first_event: ::std::os::raw::c_int,
    pub first_error: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XExtCodes"][::std::mem::size_of::<XExtCodes>() - 16usize];
    ["Alignment of XExtCodes"][::std::mem::align_of::<XExtCodes>() - 4usize];
    ["Offset of field: XExtCodes::extension"]
        [::std::mem::offset_of!(XExtCodes, extension) - 0usize];
    ["Offset of field: XExtCodes::major_opcode"]
        [::std::mem::offset_of!(XExtCodes, major_opcode) - 4usize];
    ["Offset of field: XExtCodes::first_event"]
        [::std::mem::offset_of!(XExtCodes, first_event) - 8usize];
    ["Offset of field: XExtCodes::first_error"]
        [::std::mem::offset_of!(XExtCodes, first_error) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPixmapFormatValues {
    pub depth: ::std::os::raw::c_int,
    pub bits_per_pixel: ::std::os::raw::c_int,
    pub scanline_pad: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XPixmapFormatValues"][::std::mem::size_of::<XPixmapFormatValues>() - 12usize];
    ["Alignment of XPixmapFormatValues"][::std::mem::align_of::<XPixmapFormatValues>() - 4usize];
    ["Offset of field: XPixmapFormatValues::depth"]
        [::std::mem::offset_of!(XPixmapFormatValues, depth) - 0usize];
    ["Offset of field: XPixmapFormatValues::bits_per_pixel"]
        [::std::mem::offset_of!(XPixmapFormatValues, bits_per_pixel) - 4usize];
    ["Offset of field: XPixmapFormatValues::scanline_pad"]
        [::std::mem::offset_of!(XPixmapFormatValues, scanline_pad) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGCValues {
    pub function: ::std::os::raw::c_int,
    pub plane_mask: ::std::os::raw::c_ulong,
    pub foreground: ::std::os::raw::c_ulong,
    pub background: ::std::os::raw::c_ulong,
    pub line_width: ::std::os::raw::c_int,
    pub line_style: ::std::os::raw::c_int,
    pub cap_style: ::std::os::raw::c_int,
    pub join_style: ::std::os::raw::c_int,
    pub fill_style: ::std::os::raw::c_int,
    pub fill_rule: ::std::os::raw::c_int,
    pub arc_mode: ::std::os::raw::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: ::std::os::raw::c_int,
    pub ts_y_origin: ::std::os::raw::c_int,
    pub font: Font,
    pub subwindow_mode: ::std::os::raw::c_int,
    pub graphics_exposures: ::std::os::raw::c_int,
    pub clip_x_origin: ::std::os::raw::c_int,
    pub clip_y_origin: ::std::os::raw::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: ::std::os::raw::c_int,
    pub dashes: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XGCValues"][::std::mem::size_of::<XGCValues>() - 128usize];
    ["Alignment of XGCValues"][::std::mem::align_of::<XGCValues>() - 8usize];
    ["Offset of field: XGCValues::function"][::std::mem::offset_of!(XGCValues, function) - 0usize];
    ["Offset of field: XGCValues::plane_mask"]
        [::std::mem::offset_of!(XGCValues, plane_mask) - 8usize];
    ["Offset of field: XGCValues::foreground"]
        [::std::mem::offset_of!(XGCValues, foreground) - 16usize];
    ["Offset of field: XGCValues::background"]
        [::std::mem::offset_of!(XGCValues, background) - 24usize];
    ["Offset of field: XGCValues::line_width"]
        [::std::mem::offset_of!(XGCValues, line_width) - 32usize];
    ["Offset of field: XGCValues::line_style"]
        [::std::mem::offset_of!(XGCValues, line_style) - 36usize];
    ["Offset of field: XGCValues::cap_style"]
        [::std::mem::offset_of!(XGCValues, cap_style) - 40usize];
    ["Offset of field: XGCValues::join_style"]
        [::std::mem::offset_of!(XGCValues, join_style) - 44usize];
    ["Offset of field: XGCValues::fill_style"]
        [::std::mem::offset_of!(XGCValues, fill_style) - 48usize];
    ["Offset of field: XGCValues::fill_rule"]
        [::std::mem::offset_of!(XGCValues, fill_rule) - 52usize];
    ["Offset of field: XGCValues::arc_mode"][::std::mem::offset_of!(XGCValues, arc_mode) - 56usize];
    ["Offset of field: XGCValues::tile"][::std::mem::offset_of!(XGCValues, tile) - 64usize];
    ["Offset of field: XGCValues::stipple"][::std::mem::offset_of!(XGCValues, stipple) - 72usize];
    ["Offset of field: XGCValues::ts_x_origin"]
        [::std::mem::offset_of!(XGCValues, ts_x_origin) - 80usize];
    ["Offset of field: XGCValues::ts_y_origin"]
        [::std::mem::offset_of!(XGCValues, ts_y_origin) - 84usize];
    ["Offset of field: XGCValues::font"][::std::mem::offset_of!(XGCValues, font) - 88usize];
    ["Offset of field: XGCValues::subwindow_mode"]
        [::std::mem::offset_of!(XGCValues, subwindow_mode) - 96usize];
    ["Offset of field: XGCValues::graphics_exposures"]
        [::std::mem::offset_of!(XGCValues, graphics_exposures) - 100usize];
    ["Offset of field: XGCValues::clip_x_origin"]
        [::std::mem::offset_of!(XGCValues, clip_x_origin) - 104usize];
    ["Offset of field: XGCValues::clip_y_origin"]
        [::std::mem::offset_of!(XGCValues, clip_y_origin) - 108usize];
    ["Offset of field: XGCValues::clip_mask"]
        [::std::mem::offset_of!(XGCValues, clip_mask) - 112usize];
    ["Offset of field: XGCValues::dash_offset"]
        [::std::mem::offset_of!(XGCValues, dash_offset) - 120usize];
    ["Offset of field: XGCValues::dashes"][::std::mem::offset_of!(XGCValues, dashes) - 124usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XGC {
    _unused: [u8; 0],
}
pub type GC = *mut _XGC;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: ::std::os::raw::c_int,
    pub red_mask: ::std::os::raw::c_ulong,
    pub green_mask: ::std::os::raw::c_ulong,
    pub blue_mask: ::std::os::raw::c_ulong,
    pub bits_per_rgb: ::std::os::raw::c_int,
    pub map_entries: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Visual"][::std::mem::size_of::<Visual>() - 56usize];
    ["Alignment of Visual"][::std::mem::align_of::<Visual>() - 8usize];
    ["Offset of field: Visual::ext_data"][::std::mem::offset_of!(Visual, ext_data) - 0usize];
    ["Offset of field: Visual::visualid"][::std::mem::offset_of!(Visual, visualid) - 8usize];
    ["Offset of field: Visual::class"][::std::mem::offset_of!(Visual, class) - 16usize];
    ["Offset of field: Visual::red_mask"][::std::mem::offset_of!(Visual, red_mask) - 24usize];
    ["Offset of field: Visual::green_mask"][::std::mem::offset_of!(Visual, green_mask) - 32usize];
    ["Offset of field: Visual::blue_mask"][::std::mem::offset_of!(Visual, blue_mask) - 40usize];
    ["Offset of field: Visual::bits_per_rgb"]
        [::std::mem::offset_of!(Visual, bits_per_rgb) - 48usize];
    ["Offset of field: Visual::map_entries"][::std::mem::offset_of!(Visual, map_entries) - 52usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Depth {
    pub depth: ::std::os::raw::c_int,
    pub nvisuals: ::std::os::raw::c_int,
    pub visuals: *mut Visual,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Depth"][::std::mem::size_of::<Depth>() - 16usize];
    ["Alignment of Depth"][::std::mem::align_of::<Depth>() - 8usize];
    ["Offset of field: Depth::depth"][::std::mem::offset_of!(Depth, depth) - 0usize];
    ["Offset of field: Depth::nvisuals"][::std::mem::offset_of!(Depth, nvisuals) - 4usize];
    ["Offset of field: Depth::visuals"][::std::mem::offset_of!(Depth, visuals) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XDisplay {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub mwidth: ::std::os::raw::c_int,
    pub mheight: ::std::os::raw::c_int,
    pub ndepths: ::std::os::raw::c_int,
    pub depths: *mut Depth,
    pub root_depth: ::std::os::raw::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: ::std::os::raw::c_ulong,
    pub black_pixel: ::std::os::raw::c_ulong,
    pub max_maps: ::std::os::raw::c_int,
    pub min_maps: ::std::os::raw::c_int,
    pub backing_store: ::std::os::raw::c_int,
    pub save_unders: ::std::os::raw::c_int,
    pub root_input_mask: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Screen"][::std::mem::size_of::<Screen>() - 128usize];
    ["Alignment of Screen"][::std::mem::align_of::<Screen>() - 8usize];
    ["Offset of field: Screen::ext_data"][::std::mem::offset_of!(Screen, ext_data) - 0usize];
    ["Offset of field: Screen::display"][::std::mem::offset_of!(Screen, display) - 8usize];
    ["Offset of field: Screen::root"][::std::mem::offset_of!(Screen, root) - 16usize];
    ["Offset of field: Screen::width"][::std::mem::offset_of!(Screen, width) - 24usize];
    ["Offset of field: Screen::height"][::std::mem::offset_of!(Screen, height) - 28usize];
    ["Offset of field: Screen::mwidth"][::std::mem::offset_of!(Screen, mwidth) - 32usize];
    ["Offset of field: Screen::mheight"][::std::mem::offset_of!(Screen, mheight) - 36usize];
    ["Offset of field: Screen::ndepths"][::std::mem::offset_of!(Screen, ndepths) - 40usize];
    ["Offset of field: Screen::depths"][::std::mem::offset_of!(Screen, depths) - 48usize];
    ["Offset of field: Screen::root_depth"][::std::mem::offset_of!(Screen, root_depth) - 56usize];
    ["Offset of field: Screen::root_visual"][::std::mem::offset_of!(Screen, root_visual) - 64usize];
    ["Offset of field: Screen::default_gc"][::std::mem::offset_of!(Screen, default_gc) - 72usize];
    ["Offset of field: Screen::cmap"][::std::mem::offset_of!(Screen, cmap) - 80usize];
    ["Offset of field: Screen::white_pixel"][::std::mem::offset_of!(Screen, white_pixel) - 88usize];
    ["Offset of field: Screen::black_pixel"][::std::mem::offset_of!(Screen, black_pixel) - 96usize];
    ["Offset of field: Screen::max_maps"][::std::mem::offset_of!(Screen, max_maps) - 104usize];
    ["Offset of field: Screen::min_maps"][::std::mem::offset_of!(Screen, min_maps) - 108usize];
    ["Offset of field: Screen::backing_store"]
        [::std::mem::offset_of!(Screen, backing_store) - 112usize];
    ["Offset of field: Screen::save_unders"]
        [::std::mem::offset_of!(Screen, save_unders) - 116usize];
    ["Offset of field: Screen::root_input_mask"]
        [::std::mem::offset_of!(Screen, root_input_mask) - 120usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: ::std::os::raw::c_int,
    pub bits_per_pixel: ::std::os::raw::c_int,
    pub scanline_pad: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ScreenFormat"][::std::mem::size_of::<ScreenFormat>() - 24usize];
    ["Alignment of ScreenFormat"][::std::mem::align_of::<ScreenFormat>() - 8usize];
    ["Offset of field: ScreenFormat::ext_data"]
        [::std::mem::offset_of!(ScreenFormat, ext_data) - 0usize];
    ["Offset of field: ScreenFormat::depth"][::std::mem::offset_of!(ScreenFormat, depth) - 8usize];
    ["Offset of field: ScreenFormat::bits_per_pixel"]
        [::std::mem::offset_of!(ScreenFormat, bits_per_pixel) - 12usize];
    ["Offset of field: ScreenFormat::scanline_pad"]
        [::std::mem::offset_of!(ScreenFormat, scanline_pad) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: ::std::os::raw::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: ::std::os::raw::c_ulong,
    pub bit_gravity: ::std::os::raw::c_int,
    pub win_gravity: ::std::os::raw::c_int,
    pub backing_store: ::std::os::raw::c_int,
    pub backing_planes: ::std::os::raw::c_ulong,
    pub backing_pixel: ::std::os::raw::c_ulong,
    pub save_under: ::std::os::raw::c_int,
    pub event_mask: ::std::os::raw::c_long,
    pub do_not_propagate_mask: ::std::os::raw::c_long,
    pub override_redirect: ::std::os::raw::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XSetWindowAttributes"][::std::mem::size_of::<XSetWindowAttributes>() - 112usize];
    ["Alignment of XSetWindowAttributes"][::std::mem::align_of::<XSetWindowAttributes>() - 8usize];
    ["Offset of field: XSetWindowAttributes::background_pixmap"]
        [::std::mem::offset_of!(XSetWindowAttributes, background_pixmap) - 0usize];
    ["Offset of field: XSetWindowAttributes::background_pixel"]
        [::std::mem::offset_of!(XSetWindowAttributes, background_pixel) - 8usize];
    ["Offset of field: XSetWindowAttributes::border_pixmap"]
        [::std::mem::offset_of!(XSetWindowAttributes, border_pixmap) - 16usize];
    ["Offset of field: XSetWindowAttributes::border_pixel"]
        [::std::mem::offset_of!(XSetWindowAttributes, border_pixel) - 24usize];
    ["Offset of field: XSetWindowAttributes::bit_gravity"]
        [::std::mem::offset_of!(XSetWindowAttributes, bit_gravity) - 32usize];
    ["Offset of field: XSetWindowAttributes::win_gravity"]
        [::std::mem::offset_of!(XSetWindowAttributes, win_gravity) - 36usize];
    ["Offset of field: XSetWindowAttributes::backing_store"]
        [::std::mem::offset_of!(XSetWindowAttributes, backing_store) - 40usize];
    ["Offset of field: XSetWindowAttributes::backing_planes"]
        [::std::mem::offset_of!(XSetWindowAttributes, backing_planes) - 48usize];
    ["Offset of field: XSetWindowAttributes::backing_pixel"]
        [::std::mem::offset_of!(XSetWindowAttributes, backing_pixel) - 56usize];
    ["Offset of field: XSetWindowAttributes::save_under"]
        [::std::mem::offset_of!(XSetWindowAttributes, save_under) - 64usize];
    ["Offset of field: XSetWindowAttributes::event_mask"]
        [::std::mem::offset_of!(XSetWindowAttributes, event_mask) - 72usize];
    ["Offset of field: XSetWindowAttributes::do_not_propagate_mask"]
        [::std::mem::offset_of!(XSetWindowAttributes, do_not_propagate_mask) - 80usize];
    ["Offset of field: XSetWindowAttributes::override_redirect"]
        [::std::mem::offset_of!(XSetWindowAttributes, override_redirect) - 88usize];
    ["Offset of field: XSetWindowAttributes::colormap"]
        [::std::mem::offset_of!(XSetWindowAttributes, colormap) - 96usize];
    ["Offset of field: XSetWindowAttributes::cursor"]
        [::std::mem::offset_of!(XSetWindowAttributes, cursor) - 104usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XWindowAttributes {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub border_width: ::std::os::raw::c_int,
    pub depth: ::std::os::raw::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: ::std::os::raw::c_int,
    pub bit_gravity: ::std::os::raw::c_int,
    pub win_gravity: ::std::os::raw::c_int,
    pub backing_store: ::std::os::raw::c_int,
    pub backing_planes: ::std::os::raw::c_ulong,
    pub backing_pixel: ::std::os::raw::c_ulong,
    pub save_under: ::std::os::raw::c_int,
    pub colormap: Colormap,
    pub map_installed: ::std::os::raw::c_int,
    pub map_state: ::std::os::raw::c_int,
    pub all_event_masks: ::std::os::raw::c_long,
    pub your_event_mask: ::std::os::raw::c_long,
    pub do_not_propagate_mask: ::std::os::raw::c_long,
    pub override_redirect: ::std::os::raw::c_int,
    pub screen: *mut Screen,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XWindowAttributes"][::std::mem::size_of::<XWindowAttributes>() - 136usize];
    ["Alignment of XWindowAttributes"][::std::mem::align_of::<XWindowAttributes>() - 8usize];
    ["Offset of field: XWindowAttributes::x"]
        [::std::mem::offset_of!(XWindowAttributes, x) - 0usize];
    ["Offset of field: XWindowAttributes::y"]
        [::std::mem::offset_of!(XWindowAttributes, y) - 4usize];
    ["Offset of field: XWindowAttributes::width"]
        [::std::mem::offset_of!(XWindowAttributes, width) - 8usize];
    ["Offset of field: XWindowAttributes::height"]
        [::std::mem::offset_of!(XWindowAttributes, height) - 12usize];
    ["Offset of field: XWindowAttributes::border_width"]
        [::std::mem::offset_of!(XWindowAttributes, border_width) - 16usize];
    ["Offset of field: XWindowAttributes::depth"]
        [::std::mem::offset_of!(XWindowAttributes, depth) - 20usize];
    ["Offset of field: XWindowAttributes::visual"]
        [::std::mem::offset_of!(XWindowAttributes, visual) - 24usize];
    ["Offset of field: XWindowAttributes::root"]
        [::std::mem::offset_of!(XWindowAttributes, root) - 32usize];
    ["Offset of field: XWindowAttributes::class"]
        [::std::mem::offset_of!(XWindowAttributes, class) - 40usize];
    ["Offset of field: XWindowAttributes::bit_gravity"]
        [::std::mem::offset_of!(XWindowAttributes, bit_gravity) - 44usize];
    ["Offset of field: XWindowAttributes::win_gravity"]
        [::std::mem::offset_of!(XWindowAttributes, win_gravity) - 48usize];
    ["Offset of field: XWindowAttributes::backing_store"]
        [::std::mem::offset_of!(XWindowAttributes, backing_store) - 52usize];
    ["Offset of field: XWindowAttributes::backing_planes"]
        [::std::mem::offset_of!(XWindowAttributes, backing_planes) - 56usize];
    ["Offset of field: XWindowAttributes::backing_pixel"]
        [::std::mem::offset_of!(XWindowAttributes, backing_pixel) - 64usize];
    ["Offset of field: XWindowAttributes::save_under"]
        [::std::mem::offset_of!(XWindowAttributes, save_under) - 72usize];
    ["Offset of field: XWindowAttributes::colormap"]
        [::std::mem::offset_of!(XWindowAttributes, colormap) - 80usize];
    ["Offset of field: XWindowAttributes::map_installed"]
        [::std::mem::offset_of!(XWindowAttributes, map_installed) - 88usize];
    ["Offset of field: XWindowAttributes::map_state"]
        [::std::mem::offset_of!(XWindowAttributes, map_state) - 92usize];
    ["Offset of field: XWindowAttributes::all_event_masks"]
        [::std::mem::offset_of!(XWindowAttributes, all_event_masks) - 96usize];
    ["Offset of field: XWindowAttributes::your_event_mask"]
        [::std::mem::offset_of!(XWindowAttributes, your_event_mask) - 104usize];
    ["Offset of field: XWindowAttributes::do_not_propagate_mask"]
        [::std::mem::offset_of!(XWindowAttributes, do_not_propagate_mask) - 112usize];
    ["Offset of field: XWindowAttributes::override_redirect"]
        [::std::mem::offset_of!(XWindowAttributes, override_redirect) - 120usize];
    ["Offset of field: XWindowAttributes::screen"]
        [::std::mem::offset_of!(XWindowAttributes, screen) - 128usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XHostAddress {
    pub family: ::std::os::raw::c_int,
    pub length: ::std::os::raw::c_int,
    pub address: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XHostAddress"][::std::mem::size_of::<XHostAddress>() - 16usize];
    ["Alignment of XHostAddress"][::std::mem::align_of::<XHostAddress>() - 8usize];
    ["Offset of field: XHostAddress::family"]
        [::std::mem::offset_of!(XHostAddress, family) - 0usize];
    ["Offset of field: XHostAddress::length"]
        [::std::mem::offset_of!(XHostAddress, length) - 4usize];
    ["Offset of field: XHostAddress::address"]
        [::std::mem::offset_of!(XHostAddress, address) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XServerInterpretedAddress {
    pub typelength: ::std::os::raw::c_int,
    pub valuelength: ::std::os::raw::c_int,
    pub type_: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XServerInterpretedAddress"]
        [::std::mem::size_of::<XServerInterpretedAddress>() - 24usize];
    ["Alignment of XServerInterpretedAddress"]
        [::std::mem::align_of::<XServerInterpretedAddress>() - 8usize];
    ["Offset of field: XServerInterpretedAddress::typelength"]
        [::std::mem::offset_of!(XServerInterpretedAddress, typelength) - 0usize];
    ["Offset of field: XServerInterpretedAddress::valuelength"]
        [::std::mem::offset_of!(XServerInterpretedAddress, valuelength) - 4usize];
    ["Offset of field: XServerInterpretedAddress::type_"]
        [::std::mem::offset_of!(XServerInterpretedAddress, type_) - 8usize];
    ["Offset of field: XServerInterpretedAddress::value"]
        [::std::mem::offset_of!(XServerInterpretedAddress, value) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XImage {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub xoffset: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_char,
    pub byte_order: ::std::os::raw::c_int,
    pub bitmap_unit: ::std::os::raw::c_int,
    pub bitmap_bit_order: ::std::os::raw::c_int,
    pub bitmap_pad: ::std::os::raw::c_int,
    pub depth: ::std::os::raw::c_int,
    pub bytes_per_line: ::std::os::raw::c_int,
    pub bits_per_pixel: ::std::os::raw::c_int,
    pub red_mask: ::std::os::raw::c_ulong,
    pub green_mask: ::std::os::raw::c_ulong,
    pub blue_mask: ::std::os::raw::c_ulong,
    pub obdata: XPointer,
    pub f: _XImage_funcs,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XImage_funcs {
    pub create_image: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut _XDisplay,
            arg2: *mut Visual,
            arg3: ::std::os::raw::c_uint,
            arg4: ::std::os::raw::c_int,
            arg5: ::std::os::raw::c_int,
            arg6: *mut ::std::os::raw::c_char,
            arg7: ::std::os::raw::c_uint,
            arg8: ::std::os::raw::c_uint,
            arg9: ::std::os::raw::c_int,
            arg10: ::std::os::raw::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut _XImage) -> ::std::os::raw::c_int>,
    pub get_pixel: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut _XImage,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_ulong,
    >,
    pub put_pixel: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut _XImage,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
    pub sub_image: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut _XImage,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_uint,
            arg5: ::std::os::raw::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut _XImage,
            arg2: ::std::os::raw::c_long,
        ) -> ::std::os::raw::c_int,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XImage_funcs"][::std::mem::size_of::<_XImage_funcs>() - 48usize];
    ["Alignment of _XImage_funcs"][::std::mem::align_of::<_XImage_funcs>() - 8usize];
    ["Offset of field: _XImage_funcs::create_image"]
        [::std::mem::offset_of!(_XImage_funcs, create_image) - 0usize];
    ["Offset of field: _XImage_funcs::destroy_image"]
        [::std::mem::offset_of!(_XImage_funcs, destroy_image) - 8usize];
    ["Offset of field: _XImage_funcs::get_pixel"]
        [::std::mem::offset_of!(_XImage_funcs, get_pixel) - 16usize];
    ["Offset of field: _XImage_funcs::put_pixel"]
        [::std::mem::offset_of!(_XImage_funcs, put_pixel) - 24usize];
    ["Offset of field: _XImage_funcs::sub_image"]
        [::std::mem::offset_of!(_XImage_funcs, sub_image) - 32usize];
    ["Offset of field: _XImage_funcs::add_pixel"]
        [::std::mem::offset_of!(_XImage_funcs, add_pixel) - 40usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XImage"][::std::mem::size_of::<_XImage>() - 136usize];
    ["Alignment of _XImage"][::std::mem::align_of::<_XImage>() - 8usize];
    ["Offset of field: _XImage::width"][::std::mem::offset_of!(_XImage, width) - 0usize];
    ["Offset of field: _XImage::height"][::std::mem::offset_of!(_XImage, height) - 4usize];
    ["Offset of field: _XImage::xoffset"][::std::mem::offset_of!(_XImage, xoffset) - 8usize];
    ["Offset of field: _XImage::format"][::std::mem::offset_of!(_XImage, format) - 12usize];
    ["Offset of field: _XImage::data"][::std::mem::offset_of!(_XImage, data) - 16usize];
    ["Offset of field: _XImage::byte_order"][::std::mem::offset_of!(_XImage, byte_order) - 24usize];
    ["Offset of field: _XImage::bitmap_unit"]
        [::std::mem::offset_of!(_XImage, bitmap_unit) - 28usize];
    ["Offset of field: _XImage::bitmap_bit_order"]
        [::std::mem::offset_of!(_XImage, bitmap_bit_order) - 32usize];
    ["Offset of field: _XImage::bitmap_pad"][::std::mem::offset_of!(_XImage, bitmap_pad) - 36usize];
    ["Offset of field: _XImage::depth"][::std::mem::offset_of!(_XImage, depth) - 40usize];
    ["Offset of field: _XImage::bytes_per_line"]
        [::std::mem::offset_of!(_XImage, bytes_per_line) - 44usize];
    ["Offset of field: _XImage::bits_per_pixel"]
        [::std::mem::offset_of!(_XImage, bits_per_pixel) - 48usize];
    ["Offset of field: _XImage::red_mask"][::std::mem::offset_of!(_XImage, red_mask) - 56usize];
    ["Offset of field: _XImage::green_mask"][::std::mem::offset_of!(_XImage, green_mask) - 64usize];
    ["Offset of field: _XImage::blue_mask"][::std::mem::offset_of!(_XImage, blue_mask) - 72usize];
    ["Offset of field: _XImage::obdata"][::std::mem::offset_of!(_XImage, obdata) - 80usize];
    ["Offset of field: _XImage::f"][::std::mem::offset_of!(_XImage, f) - 88usize];
};
pub type XImage = _XImage;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XWindowChanges {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub border_width: ::std::os::raw::c_int,
    pub sibling: Window,
    pub stack_mode: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XWindowChanges"][::std::mem::size_of::<XWindowChanges>() - 40usize];
    ["Alignment of XWindowChanges"][::std::mem::align_of::<XWindowChanges>() - 8usize];
    ["Offset of field: XWindowChanges::x"][::std::mem::offset_of!(XWindowChanges, x) - 0usize];
    ["Offset of field: XWindowChanges::y"][::std::mem::offset_of!(XWindowChanges, y) - 4usize];
    ["Offset of field: XWindowChanges::width"]
        [::std::mem::offset_of!(XWindowChanges, width) - 8usize];
    ["Offset of field: XWindowChanges::height"]
        [::std::mem::offset_of!(XWindowChanges, height) - 12usize];
    ["Offset of field: XWindowChanges::border_width"]
        [::std::mem::offset_of!(XWindowChanges, border_width) - 16usize];
    ["Offset of field: XWindowChanges::sibling"]
        [::std::mem::offset_of!(XWindowChanges, sibling) - 24usize];
    ["Offset of field: XWindowChanges::stack_mode"]
        [::std::mem::offset_of!(XWindowChanges, stack_mode) - 32usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XColor {
    pub pixel: ::std::os::raw::c_ulong,
    pub red: ::std::os::raw::c_ushort,
    pub green: ::std::os::raw::c_ushort,
    pub blue: ::std::os::raw::c_ushort,
    pub flags: ::std::os::raw::c_char,
    pub pad: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XColor"][::std::mem::size_of::<XColor>() - 16usize];
    ["Alignment of XColor"][::std::mem::align_of::<XColor>() - 8usize];
    ["Offset of field: XColor::pixel"][::std::mem::offset_of!(XColor, pixel) - 0usize];
    ["Offset of field: XColor::red"][::std::mem::offset_of!(XColor, red) - 8usize];
    ["Offset of field: XColor::green"][::std::mem::offset_of!(XColor, green) - 10usize];
    ["Offset of field: XColor::blue"][::std::mem::offset_of!(XColor, blue) - 12usize];
    ["Offset of field: XColor::flags"][::std::mem::offset_of!(XColor, flags) - 14usize];
    ["Offset of field: XColor::pad"][::std::mem::offset_of!(XColor, pad) - 15usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSegment {
    pub x1: ::std::os::raw::c_short,
    pub y1: ::std::os::raw::c_short,
    pub x2: ::std::os::raw::c_short,
    pub y2: ::std::os::raw::c_short,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XSegment"][::std::mem::size_of::<XSegment>() - 8usize];
    ["Alignment of XSegment"][::std::mem::align_of::<XSegment>() - 2usize];
    ["Offset of field: XSegment::x1"][::std::mem::offset_of!(XSegment, x1) - 0usize];
    ["Offset of field: XSegment::y1"][::std::mem::offset_of!(XSegment, y1) - 2usize];
    ["Offset of field: XSegment::x2"][::std::mem::offset_of!(XSegment, x2) - 4usize];
    ["Offset of field: XSegment::y2"][::std::mem::offset_of!(XSegment, y2) - 6usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPoint {
    pub x: ::std::os::raw::c_short,
    pub y: ::std::os::raw::c_short,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XPoint"][::std::mem::size_of::<XPoint>() - 4usize];
    ["Alignment of XPoint"][::std::mem::align_of::<XPoint>() - 2usize];
    ["Offset of field: XPoint::x"][::std::mem::offset_of!(XPoint, x) - 0usize];
    ["Offset of field: XPoint::y"][::std::mem::offset_of!(XPoint, y) - 2usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XRectangle {
    pub x: ::std::os::raw::c_short,
    pub y: ::std::os::raw::c_short,
    pub width: ::std::os::raw::c_ushort,
    pub height: ::std::os::raw::c_ushort,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XRectangle"][::std::mem::size_of::<XRectangle>() - 8usize];
    ["Alignment of XRectangle"][::std::mem::align_of::<XRectangle>() - 2usize];
    ["Offset of field: XRectangle::x"][::std::mem::offset_of!(XRectangle, x) - 0usize];
    ["Offset of field: XRectangle::y"][::std::mem::offset_of!(XRectangle, y) - 2usize];
    ["Offset of field: XRectangle::width"][::std::mem::offset_of!(XRectangle, width) - 4usize];
    ["Offset of field: XRectangle::height"][::std::mem::offset_of!(XRectangle, height) - 6usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XArc {
    pub x: ::std::os::raw::c_short,
    pub y: ::std::os::raw::c_short,
    pub width: ::std::os::raw::c_ushort,
    pub height: ::std::os::raw::c_ushort,
    pub angle1: ::std::os::raw::c_short,
    pub angle2: ::std::os::raw::c_short,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XArc"][::std::mem::size_of::<XArc>() - 12usize];
    ["Alignment of XArc"][::std::mem::align_of::<XArc>() - 2usize];
    ["Offset of field: XArc::x"][::std::mem::offset_of!(XArc, x) - 0usize];
    ["Offset of field: XArc::y"][::std::mem::offset_of!(XArc, y) - 2usize];
    ["Offset of field: XArc::width"][::std::mem::offset_of!(XArc, width) - 4usize];
    ["Offset of field: XArc::height"][::std::mem::offset_of!(XArc, height) - 6usize];
    ["Offset of field: XArc::angle1"][::std::mem::offset_of!(XArc, angle1) - 8usize];
    ["Offset of field: XArc::angle2"][::std::mem::offset_of!(XArc, angle2) - 10usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeyboardControl {
    pub key_click_percent: ::std::os::raw::c_int,
    pub bell_percent: ::std::os::raw::c_int,
    pub bell_pitch: ::std::os::raw::c_int,
    pub bell_duration: ::std::os::raw::c_int,
    pub led: ::std::os::raw::c_int,
    pub led_mode: ::std::os::raw::c_int,
    pub key: ::std::os::raw::c_int,
    pub auto_repeat_mode: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XKeyboardControl"][::std::mem::size_of::<XKeyboardControl>() - 32usize];
    ["Alignment of XKeyboardControl"][::std::mem::align_of::<XKeyboardControl>() - 4usize];
    ["Offset of field: XKeyboardControl::key_click_percent"]
        [::std::mem::offset_of!(XKeyboardControl, key_click_percent) - 0usize];
    ["Offset of field: XKeyboardControl::bell_percent"]
        [::std::mem::offset_of!(XKeyboardControl, bell_percent) - 4usize];
    ["Offset of field: XKeyboardControl::bell_pitch"]
        [::std::mem::offset_of!(XKeyboardControl, bell_pitch) - 8usize];
    ["Offset of field: XKeyboardControl::bell_duration"]
        [::std::mem::offset_of!(XKeyboardControl, bell_duration) - 12usize];
    ["Offset of field: XKeyboardControl::led"]
        [::std::mem::offset_of!(XKeyboardControl, led) - 16usize];
    ["Offset of field: XKeyboardControl::led_mode"]
        [::std::mem::offset_of!(XKeyboardControl, led_mode) - 20usize];
    ["Offset of field: XKeyboardControl::key"]
        [::std::mem::offset_of!(XKeyboardControl, key) - 24usize];
    ["Offset of field: XKeyboardControl::auto_repeat_mode"]
        [::std::mem::offset_of!(XKeyboardControl, auto_repeat_mode) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeyboardState {
    pub key_click_percent: ::std::os::raw::c_int,
    pub bell_percent: ::std::os::raw::c_int,
    pub bell_pitch: ::std::os::raw::c_uint,
    pub bell_duration: ::std::os::raw::c_uint,
    pub led_mask: ::std::os::raw::c_ulong,
    pub global_auto_repeat: ::std::os::raw::c_int,
    pub auto_repeats: [::std::os::raw::c_char; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XKeyboardState"][::std::mem::size_of::<XKeyboardState>() - 64usize];
    ["Alignment of XKeyboardState"][::std::mem::align_of::<XKeyboardState>() - 8usize];
    ["Offset of field: XKeyboardState::key_click_percent"]
        [::std::mem::offset_of!(XKeyboardState, key_click_percent) - 0usize];
    ["Offset of field: XKeyboardState::bell_percent"]
        [::std::mem::offset_of!(XKeyboardState, bell_percent) - 4usize];
    ["Offset of field: XKeyboardState::bell_pitch"]
        [::std::mem::offset_of!(XKeyboardState, bell_pitch) - 8usize];
    ["Offset of field: XKeyboardState::bell_duration"]
        [::std::mem::offset_of!(XKeyboardState, bell_duration) - 12usize];
    ["Offset of field: XKeyboardState::led_mask"]
        [::std::mem::offset_of!(XKeyboardState, led_mask) - 16usize];
    ["Offset of field: XKeyboardState::global_auto_repeat"]
        [::std::mem::offset_of!(XKeyboardState, global_auto_repeat) - 24usize];
    ["Offset of field: XKeyboardState::auto_repeats"]
        [::std::mem::offset_of!(XKeyboardState, auto_repeats) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XTimeCoord {
    pub time: Time,
    pub x: ::std::os::raw::c_short,
    pub y: ::std::os::raw::c_short,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XTimeCoord"][::std::mem::size_of::<XTimeCoord>() - 16usize];
    ["Alignment of XTimeCoord"][::std::mem::align_of::<XTimeCoord>() - 8usize];
    ["Offset of field: XTimeCoord::time"][::std::mem::offset_of!(XTimeCoord, time) - 0usize];
    ["Offset of field: XTimeCoord::x"][::std::mem::offset_of!(XTimeCoord, x) - 8usize];
    ["Offset of field: XTimeCoord::y"][::std::mem::offset_of!(XTimeCoord, y) - 10usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XModifierKeymap {
    pub max_keypermod: ::std::os::raw::c_int,
    pub modifiermap: *mut KeyCode,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XModifierKeymap"][::std::mem::size_of::<XModifierKeymap>() - 16usize];
    ["Alignment of XModifierKeymap"][::std::mem::align_of::<XModifierKeymap>() - 8usize];
    ["Offset of field: XModifierKeymap::max_keypermod"]
        [::std::mem::offset_of!(XModifierKeymap, max_keypermod) - 0usize];
    ["Offset of field: XModifierKeymap::modifiermap"]
        [::std::mem::offset_of!(XModifierKeymap, modifiermap) - 8usize];
};
pub type Display = _XDisplay;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XPrivate {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XrmHashBucketRec {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: ::std::os::raw::c_int,
    pub private2: ::std::os::raw::c_int,
    pub proto_major_version: ::std::os::raw::c_int,
    pub proto_minor_version: ::std::os::raw::c_int,
    pub vendor: *mut ::std::os::raw::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: ::std::os::raw::c_int,
    pub resource_alloc: ::std::option::Option<unsafe extern "C" fn(arg1: *mut _XDisplay) -> XID>,
    pub byte_order: ::std::os::raw::c_int,
    pub bitmap_unit: ::std::os::raw::c_int,
    pub bitmap_pad: ::std::os::raw::c_int,
    pub bitmap_bit_order: ::std::os::raw::c_int,
    pub nformats: ::std::os::raw::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: ::std::os::raw::c_int,
    pub release: ::std::os::raw::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: ::std::os::raw::c_int,
    pub last_request_read: ::std::os::raw::c_ulong,
    pub request: ::std::os::raw::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: ::std::os::raw::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut _XDisplay) -> ::std::os::raw::c_int>,
    pub display_name: *mut ::std::os::raw::c_char,
    pub default_screen: ::std::os::raw::c_int,
    pub nscreens: ::std::os::raw::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: ::std::os::raw::c_ulong,
    pub private16: ::std::os::raw::c_ulong,
    pub min_keycode: ::std::os::raw::c_int,
    pub max_keycode: ::std::os::raw::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: ::std::os::raw::c_int,
    pub xdefaults: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _bindgen_ty_1"][::std::mem::size_of::<_bindgen_ty_1>() - 296usize];
    ["Alignment of _bindgen_ty_1"][::std::mem::align_of::<_bindgen_ty_1>() - 8usize];
    ["Offset of field: _bindgen_ty_1::ext_data"]
        [::std::mem::offset_of!(_bindgen_ty_1, ext_data) - 0usize];
    ["Offset of field: _bindgen_ty_1::private1"]
        [::std::mem::offset_of!(_bindgen_ty_1, private1) - 8usize];
    ["Offset of field: _bindgen_ty_1::fd"][::std::mem::offset_of!(_bindgen_ty_1, fd) - 16usize];
    ["Offset of field: _bindgen_ty_1::private2"]
        [::std::mem::offset_of!(_bindgen_ty_1, private2) - 20usize];
    ["Offset of field: _bindgen_ty_1::proto_major_version"]
        [::std::mem::offset_of!(_bindgen_ty_1, proto_major_version) - 24usize];
    ["Offset of field: _bindgen_ty_1::proto_minor_version"]
        [::std::mem::offset_of!(_bindgen_ty_1, proto_minor_version) - 28usize];
    ["Offset of field: _bindgen_ty_1::vendor"]
        [::std::mem::offset_of!(_bindgen_ty_1, vendor) - 32usize];
    ["Offset of field: _bindgen_ty_1::private3"]
        [::std::mem::offset_of!(_bindgen_ty_1, private3) - 40usize];
    ["Offset of field: _bindgen_ty_1::private4"]
        [::std::mem::offset_of!(_bindgen_ty_1, private4) - 48usize];
    ["Offset of field: _bindgen_ty_1::private5"]
        [::std::mem::offset_of!(_bindgen_ty_1, private5) - 56usize];
    ["Offset of field: _bindgen_ty_1::private6"]
        [::std::mem::offset_of!(_bindgen_ty_1, private6) - 64usize];
    ["Offset of field: _bindgen_ty_1::resource_alloc"]
        [::std::mem::offset_of!(_bindgen_ty_1, resource_alloc) - 72usize];
    ["Offset of field: _bindgen_ty_1::byte_order"]
        [::std::mem::offset_of!(_bindgen_ty_1, byte_order) - 80usize];
    ["Offset of field: _bindgen_ty_1::bitmap_unit"]
        [::std::mem::offset_of!(_bindgen_ty_1, bitmap_unit) - 84usize];
    ["Offset of field: _bindgen_ty_1::bitmap_pad"]
        [::std::mem::offset_of!(_bindgen_ty_1, bitmap_pad) - 88usize];
    ["Offset of field: _bindgen_ty_1::bitmap_bit_order"]
        [::std::mem::offset_of!(_bindgen_ty_1, bitmap_bit_order) - 92usize];
    ["Offset of field: _bindgen_ty_1::nformats"]
        [::std::mem::offset_of!(_bindgen_ty_1, nformats) - 96usize];
    ["Offset of field: _bindgen_ty_1::pixmap_format"]
        [::std::mem::offset_of!(_bindgen_ty_1, pixmap_format) - 104usize];
    ["Offset of field: _bindgen_ty_1::private8"]
        [::std::mem::offset_of!(_bindgen_ty_1, private8) - 112usize];
    ["Offset of field: _bindgen_ty_1::release"]
        [::std::mem::offset_of!(_bindgen_ty_1, release) - 116usize];
    ["Offset of field: _bindgen_ty_1::private9"]
        [::std::mem::offset_of!(_bindgen_ty_1, private9) - 120usize];
    ["Offset of field: _bindgen_ty_1::private10"]
        [::std::mem::offset_of!(_bindgen_ty_1, private10) - 128usize];
    ["Offset of field: _bindgen_ty_1::qlen"]
        [::std::mem::offset_of!(_bindgen_ty_1, qlen) - 136usize];
    ["Offset of field: _bindgen_ty_1::last_request_read"]
        [::std::mem::offset_of!(_bindgen_ty_1, last_request_read) - 144usize];
    ["Offset of field: _bindgen_ty_1::request"]
        [::std::mem::offset_of!(_bindgen_ty_1, request) - 152usize];
    ["Offset of field: _bindgen_ty_1::private11"]
        [::std::mem::offset_of!(_bindgen_ty_1, private11) - 160usize];
    ["Offset of field: _bindgen_ty_1::private12"]
        [::std::mem::offset_of!(_bindgen_ty_1, private12) - 168usize];
    ["Offset of field: _bindgen_ty_1::private13"]
        [::std::mem::offset_of!(_bindgen_ty_1, private13) - 176usize];
    ["Offset of field: _bindgen_ty_1::private14"]
        [::std::mem::offset_of!(_bindgen_ty_1, private14) - 184usize];
    ["Offset of field: _bindgen_ty_1::max_request_size"]
        [::std::mem::offset_of!(_bindgen_ty_1, max_request_size) - 192usize];
    ["Offset of field: _bindgen_ty_1::db"][::std::mem::offset_of!(_bindgen_ty_1, db) - 200usize];
    ["Offset of field: _bindgen_ty_1::private15"]
        [::std::mem::offset_of!(_bindgen_ty_1, private15) - 208usize];
    ["Offset of field: _bindgen_ty_1::display_name"]
        [::std::mem::offset_of!(_bindgen_ty_1, display_name) - 216usize];
    ["Offset of field: _bindgen_ty_1::default_screen"]
        [::std::mem::offset_of!(_bindgen_ty_1, default_screen) - 224usize];
    ["Offset of field: _bindgen_ty_1::nscreens"]
        [::std::mem::offset_of!(_bindgen_ty_1, nscreens) - 228usize];
    ["Offset of field: _bindgen_ty_1::screens"]
        [::std::mem::offset_of!(_bindgen_ty_1, screens) - 232usize];
    ["Offset of field: _bindgen_ty_1::motion_buffer"]
        [::std::mem::offset_of!(_bindgen_ty_1, motion_buffer) - 240usize];
    ["Offset of field: _bindgen_ty_1::private16"]
        [::std::mem::offset_of!(_bindgen_ty_1, private16) - 248usize];
    ["Offset of field: _bindgen_ty_1::min_keycode"]
        [::std::mem::offset_of!(_bindgen_ty_1, min_keycode) - 256usize];
    ["Offset of field: _bindgen_ty_1::max_keycode"]
        [::std::mem::offset_of!(_bindgen_ty_1, max_keycode) - 260usize];
    ["Offset of field: _bindgen_ty_1::private17"]
        [::std::mem::offset_of!(_bindgen_ty_1, private17) - 264usize];
    ["Offset of field: _bindgen_ty_1::private18"]
        [::std::mem::offset_of!(_bindgen_ty_1, private18) - 272usize];
    ["Offset of field: _bindgen_ty_1::private19"]
        [::std::mem::offset_of!(_bindgen_ty_1, private19) - 280usize];
    ["Offset of field: _bindgen_ty_1::xdefaults"]
        [::std::mem::offset_of!(_bindgen_ty_1, xdefaults) - 288usize];
};
pub type _XPrivDisplay = *mut _bindgen_ty_1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeyEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub x_root: ::std::os::raw::c_int,
    pub y_root: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_uint,
    pub keycode: ::std::os::raw::c_uint,
    pub same_screen: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XKeyEvent"][::std::mem::size_of::<XKeyEvent>() - 96usize];
    ["Alignment of XKeyEvent"][::std::mem::align_of::<XKeyEvent>() - 8usize];
    ["Offset of field: XKeyEvent::type_"][::std::mem::offset_of!(XKeyEvent, type_) - 0usize];
    ["Offset of field: XKeyEvent::serial"][::std::mem::offset_of!(XKeyEvent, serial) - 8usize];
    ["Offset of field: XKeyEvent::send_event"]
        [::std::mem::offset_of!(XKeyEvent, send_event) - 16usize];
    ["Offset of field: XKeyEvent::display"][::std::mem::offset_of!(XKeyEvent, display) - 24usize];
    ["Offset of field: XKeyEvent::window"][::std::mem::offset_of!(XKeyEvent, window) - 32usize];
    ["Offset of field: XKeyEvent::root"][::std::mem::offset_of!(XKeyEvent, root) - 40usize];
    ["Offset of field: XKeyEvent::subwindow"]
        [::std::mem::offset_of!(XKeyEvent, subwindow) - 48usize];
    ["Offset of field: XKeyEvent::time"][::std::mem::offset_of!(XKeyEvent, time) - 56usize];
    ["Offset of field: XKeyEvent::x"][::std::mem::offset_of!(XKeyEvent, x) - 64usize];
    ["Offset of field: XKeyEvent::y"][::std::mem::offset_of!(XKeyEvent, y) - 68usize];
    ["Offset of field: XKeyEvent::x_root"][::std::mem::offset_of!(XKeyEvent, x_root) - 72usize];
    ["Offset of field: XKeyEvent::y_root"][::std::mem::offset_of!(XKeyEvent, y_root) - 76usize];
    ["Offset of field: XKeyEvent::state"][::std::mem::offset_of!(XKeyEvent, state) - 80usize];
    ["Offset of field: XKeyEvent::keycode"][::std::mem::offset_of!(XKeyEvent, keycode) - 84usize];
    ["Offset of field: XKeyEvent::same_screen"]
        [::std::mem::offset_of!(XKeyEvent, same_screen) - 88usize];
};
pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XButtonEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub x_root: ::std::os::raw::c_int,
    pub y_root: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_uint,
    pub button: ::std::os::raw::c_uint,
    pub same_screen: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XButtonEvent"][::std::mem::size_of::<XButtonEvent>() - 96usize];
    ["Alignment of XButtonEvent"][::std::mem::align_of::<XButtonEvent>() - 8usize];
    ["Offset of field: XButtonEvent::type_"][::std::mem::offset_of!(XButtonEvent, type_) - 0usize];
    ["Offset of field: XButtonEvent::serial"]
        [::std::mem::offset_of!(XButtonEvent, serial) - 8usize];
    ["Offset of field: XButtonEvent::send_event"]
        [::std::mem::offset_of!(XButtonEvent, send_event) - 16usize];
    ["Offset of field: XButtonEvent::display"]
        [::std::mem::offset_of!(XButtonEvent, display) - 24usize];
    ["Offset of field: XButtonEvent::window"]
        [::std::mem::offset_of!(XButtonEvent, window) - 32usize];
    ["Offset of field: XButtonEvent::root"][::std::mem::offset_of!(XButtonEvent, root) - 40usize];
    ["Offset of field: XButtonEvent::subwindow"]
        [::std::mem::offset_of!(XButtonEvent, subwindow) - 48usize];
    ["Offset of field: XButtonEvent::time"][::std::mem::offset_of!(XButtonEvent, time) - 56usize];
    ["Offset of field: XButtonEvent::x"][::std::mem::offset_of!(XButtonEvent, x) - 64usize];
    ["Offset of field: XButtonEvent::y"][::std::mem::offset_of!(XButtonEvent, y) - 68usize];
    ["Offset of field: XButtonEvent::x_root"]
        [::std::mem::offset_of!(XButtonEvent, x_root) - 72usize];
    ["Offset of field: XButtonEvent::y_root"]
        [::std::mem::offset_of!(XButtonEvent, y_root) - 76usize];
    ["Offset of field: XButtonEvent::state"][::std::mem::offset_of!(XButtonEvent, state) - 80usize];
    ["Offset of field: XButtonEvent::button"]
        [::std::mem::offset_of!(XButtonEvent, button) - 84usize];
    ["Offset of field: XButtonEvent::same_screen"]
        [::std::mem::offset_of!(XButtonEvent, same_screen) - 88usize];
};
pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMotionEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub x_root: ::std::os::raw::c_int,
    pub y_root: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_uint,
    pub is_hint: ::std::os::raw::c_char,
    pub same_screen: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XMotionEvent"][::std::mem::size_of::<XMotionEvent>() - 96usize];
    ["Alignment of XMotionEvent"][::std::mem::align_of::<XMotionEvent>() - 8usize];
    ["Offset of field: XMotionEvent::type_"][::std::mem::offset_of!(XMotionEvent, type_) - 0usize];
    ["Offset of field: XMotionEvent::serial"]
        [::std::mem::offset_of!(XMotionEvent, serial) - 8usize];
    ["Offset of field: XMotionEvent::send_event"]
        [::std::mem::offset_of!(XMotionEvent, send_event) - 16usize];
    ["Offset of field: XMotionEvent::display"]
        [::std::mem::offset_of!(XMotionEvent, display) - 24usize];
    ["Offset of field: XMotionEvent::window"]
        [::std::mem::offset_of!(XMotionEvent, window) - 32usize];
    ["Offset of field: XMotionEvent::root"][::std::mem::offset_of!(XMotionEvent, root) - 40usize];
    ["Offset of field: XMotionEvent::subwindow"]
        [::std::mem::offset_of!(XMotionEvent, subwindow) - 48usize];
    ["Offset of field: XMotionEvent::time"][::std::mem::offset_of!(XMotionEvent, time) - 56usize];
    ["Offset of field: XMotionEvent::x"][::std::mem::offset_of!(XMotionEvent, x) - 64usize];
    ["Offset of field: XMotionEvent::y"][::std::mem::offset_of!(XMotionEvent, y) - 68usize];
    ["Offset of field: XMotionEvent::x_root"]
        [::std::mem::offset_of!(XMotionEvent, x_root) - 72usize];
    ["Offset of field: XMotionEvent::y_root"]
        [::std::mem::offset_of!(XMotionEvent, y_root) - 76usize];
    ["Offset of field: XMotionEvent::state"][::std::mem::offset_of!(XMotionEvent, state) - 80usize];
    ["Offset of field: XMotionEvent::is_hint"]
        [::std::mem::offset_of!(XMotionEvent, is_hint) - 84usize];
    ["Offset of field: XMotionEvent::same_screen"]
        [::std::mem::offset_of!(XMotionEvent, same_screen) - 88usize];
};
pub type XPointerMovedEvent = XMotionEvent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCrossingEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub x_root: ::std::os::raw::c_int,
    pub y_root: ::std::os::raw::c_int,
    pub mode: ::std::os::raw::c_int,
    pub detail: ::std::os::raw::c_int,
    pub same_screen: ::std::os::raw::c_int,
    pub focus: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XCrossingEvent"][::std::mem::size_of::<XCrossingEvent>() - 104usize];
    ["Alignment of XCrossingEvent"][::std::mem::align_of::<XCrossingEvent>() - 8usize];
    ["Offset of field: XCrossingEvent::type_"]
        [::std::mem::offset_of!(XCrossingEvent, type_) - 0usize];
    ["Offset of field: XCrossingEvent::serial"]
        [::std::mem::offset_of!(XCrossingEvent, serial) - 8usize];
    ["Offset of field: XCrossingEvent::send_event"]
        [::std::mem::offset_of!(XCrossingEvent, send_event) - 16usize];
    ["Offset of field: XCrossingEvent::display"]
        [::std::mem::offset_of!(XCrossingEvent, display) - 24usize];
    ["Offset of field: XCrossingEvent::window"]
        [::std::mem::offset_of!(XCrossingEvent, window) - 32usize];
    ["Offset of field: XCrossingEvent::root"]
        [::std::mem::offset_of!(XCrossingEvent, root) - 40usize];
    ["Offset of field: XCrossingEvent::subwindow"]
        [::std::mem::offset_of!(XCrossingEvent, subwindow) - 48usize];
    ["Offset of field: XCrossingEvent::time"]
        [::std::mem::offset_of!(XCrossingEvent, time) - 56usize];
    ["Offset of field: XCrossingEvent::x"][::std::mem::offset_of!(XCrossingEvent, x) - 64usize];
    ["Offset of field: XCrossingEvent::y"][::std::mem::offset_of!(XCrossingEvent, y) - 68usize];
    ["Offset of field: XCrossingEvent::x_root"]
        [::std::mem::offset_of!(XCrossingEvent, x_root) - 72usize];
    ["Offset of field: XCrossingEvent::y_root"]
        [::std::mem::offset_of!(XCrossingEvent, y_root) - 76usize];
    ["Offset of field: XCrossingEvent::mode"]
        [::std::mem::offset_of!(XCrossingEvent, mode) - 80usize];
    ["Offset of field: XCrossingEvent::detail"]
        [::std::mem::offset_of!(XCrossingEvent, detail) - 84usize];
    ["Offset of field: XCrossingEvent::same_screen"]
        [::std::mem::offset_of!(XCrossingEvent, same_screen) - 88usize];
    ["Offset of field: XCrossingEvent::focus"]
        [::std::mem::offset_of!(XCrossingEvent, focus) - 92usize];
    ["Offset of field: XCrossingEvent::state"]
        [::std::mem::offset_of!(XCrossingEvent, state) - 96usize];
};
pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFocusChangeEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: ::std::os::raw::c_int,
    pub detail: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XFocusChangeEvent"][::std::mem::size_of::<XFocusChangeEvent>() - 48usize];
    ["Alignment of XFocusChangeEvent"][::std::mem::align_of::<XFocusChangeEvent>() - 8usize];
    ["Offset of field: XFocusChangeEvent::type_"]
        [::std::mem::offset_of!(XFocusChangeEvent, type_) - 0usize];
    ["Offset of field: XFocusChangeEvent::serial"]
        [::std::mem::offset_of!(XFocusChangeEvent, serial) - 8usize];
    ["Offset of field: XFocusChangeEvent::send_event"]
        [::std::mem::offset_of!(XFocusChangeEvent, send_event) - 16usize];
    ["Offset of field: XFocusChangeEvent::display"]
        [::std::mem::offset_of!(XFocusChangeEvent, display) - 24usize];
    ["Offset of field: XFocusChangeEvent::window"]
        [::std::mem::offset_of!(XFocusChangeEvent, window) - 32usize];
    ["Offset of field: XFocusChangeEvent::mode"]
        [::std::mem::offset_of!(XFocusChangeEvent, mode) - 40usize];
    ["Offset of field: XFocusChangeEvent::detail"]
        [::std::mem::offset_of!(XFocusChangeEvent, detail) - 44usize];
};
pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XKeymapEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [::std::os::raw::c_char; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XKeymapEvent"][::std::mem::size_of::<XKeymapEvent>() - 72usize];
    ["Alignment of XKeymapEvent"][::std::mem::align_of::<XKeymapEvent>() - 8usize];
    ["Offset of field: XKeymapEvent::type_"][::std::mem::offset_of!(XKeymapEvent, type_) - 0usize];
    ["Offset of field: XKeymapEvent::serial"]
        [::std::mem::offset_of!(XKeymapEvent, serial) - 8usize];
    ["Offset of field: XKeymapEvent::send_event"]
        [::std::mem::offset_of!(XKeymapEvent, send_event) - 16usize];
    ["Offset of field: XKeymapEvent::display"]
        [::std::mem::offset_of!(XKeymapEvent, display) - 24usize];
    ["Offset of field: XKeymapEvent::window"]
        [::std::mem::offset_of!(XKeymapEvent, window) - 32usize];
    ["Offset of field: XKeymapEvent::key_vector"]
        [::std::mem::offset_of!(XKeymapEvent, key_vector) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XExposeEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub count: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XExposeEvent"][::std::mem::size_of::<XExposeEvent>() - 64usize];
    ["Alignment of XExposeEvent"][::std::mem::align_of::<XExposeEvent>() - 8usize];
    ["Offset of field: XExposeEvent::type_"][::std::mem::offset_of!(XExposeEvent, type_) - 0usize];
    ["Offset of field: XExposeEvent::serial"]
        [::std::mem::offset_of!(XExposeEvent, serial) - 8usize];
    ["Offset of field: XExposeEvent::send_event"]
        [::std::mem::offset_of!(XExposeEvent, send_event) - 16usize];
    ["Offset of field: XExposeEvent::display"]
        [::std::mem::offset_of!(XExposeEvent, display) - 24usize];
    ["Offset of field: XExposeEvent::window"]
        [::std::mem::offset_of!(XExposeEvent, window) - 32usize];
    ["Offset of field: XExposeEvent::x"][::std::mem::offset_of!(XExposeEvent, x) - 40usize];
    ["Offset of field: XExposeEvent::y"][::std::mem::offset_of!(XExposeEvent, y) - 44usize];
    ["Offset of field: XExposeEvent::width"][::std::mem::offset_of!(XExposeEvent, width) - 48usize];
    ["Offset of field: XExposeEvent::height"]
        [::std::mem::offset_of!(XExposeEvent, height) - 52usize];
    ["Offset of field: XExposeEvent::count"][::std::mem::offset_of!(XExposeEvent, count) - 56usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGraphicsExposeEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub count: ::std::os::raw::c_int,
    pub major_code: ::std::os::raw::c_int,
    pub minor_code: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XGraphicsExposeEvent"][::std::mem::size_of::<XGraphicsExposeEvent>() - 72usize];
    ["Alignment of XGraphicsExposeEvent"][::std::mem::align_of::<XGraphicsExposeEvent>() - 8usize];
    ["Offset of field: XGraphicsExposeEvent::type_"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, type_) - 0usize];
    ["Offset of field: XGraphicsExposeEvent::serial"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, serial) - 8usize];
    ["Offset of field: XGraphicsExposeEvent::send_event"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, send_event) - 16usize];
    ["Offset of field: XGraphicsExposeEvent::display"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, display) - 24usize];
    ["Offset of field: XGraphicsExposeEvent::drawable"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, drawable) - 32usize];
    ["Offset of field: XGraphicsExposeEvent::x"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, x) - 40usize];
    ["Offset of field: XGraphicsExposeEvent::y"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, y) - 44usize];
    ["Offset of field: XGraphicsExposeEvent::width"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, width) - 48usize];
    ["Offset of field: XGraphicsExposeEvent::height"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, height) - 52usize];
    ["Offset of field: XGraphicsExposeEvent::count"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, count) - 56usize];
    ["Offset of field: XGraphicsExposeEvent::major_code"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, major_code) - 60usize];
    ["Offset of field: XGraphicsExposeEvent::minor_code"]
        [::std::mem::offset_of!(XGraphicsExposeEvent, minor_code) - 64usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XNoExposeEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: ::std::os::raw::c_int,
    pub minor_code: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XNoExposeEvent"][::std::mem::size_of::<XNoExposeEvent>() - 48usize];
    ["Alignment of XNoExposeEvent"][::std::mem::align_of::<XNoExposeEvent>() - 8usize];
    ["Offset of field: XNoExposeEvent::type_"]
        [::std::mem::offset_of!(XNoExposeEvent, type_) - 0usize];
    ["Offset of field: XNoExposeEvent::serial"]
        [::std::mem::offset_of!(XNoExposeEvent, serial) - 8usize];
    ["Offset of field: XNoExposeEvent::send_event"]
        [::std::mem::offset_of!(XNoExposeEvent, send_event) - 16usize];
    ["Offset of field: XNoExposeEvent::display"]
        [::std::mem::offset_of!(XNoExposeEvent, display) - 24usize];
    ["Offset of field: XNoExposeEvent::drawable"]
        [::std::mem::offset_of!(XNoExposeEvent, drawable) - 32usize];
    ["Offset of field: XNoExposeEvent::major_code"]
        [::std::mem::offset_of!(XNoExposeEvent, major_code) - 40usize];
    ["Offset of field: XNoExposeEvent::minor_code"]
        [::std::mem::offset_of!(XNoExposeEvent, minor_code) - 44usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XVisibilityEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XVisibilityEvent"][::std::mem::size_of::<XVisibilityEvent>() - 48usize];
    ["Alignment of XVisibilityEvent"][::std::mem::align_of::<XVisibilityEvent>() - 8usize];
    ["Offset of field: XVisibilityEvent::type_"]
        [::std::mem::offset_of!(XVisibilityEvent, type_) - 0usize];
    ["Offset of field: XVisibilityEvent::serial"]
        [::std::mem::offset_of!(XVisibilityEvent, serial) - 8usize];
    ["Offset of field: XVisibilityEvent::send_event"]
        [::std::mem::offset_of!(XVisibilityEvent, send_event) - 16usize];
    ["Offset of field: XVisibilityEvent::display"]
        [::std::mem::offset_of!(XVisibilityEvent, display) - 24usize];
    ["Offset of field: XVisibilityEvent::window"]
        [::std::mem::offset_of!(XVisibilityEvent, window) - 32usize];
    ["Offset of field: XVisibilityEvent::state"]
        [::std::mem::offset_of!(XVisibilityEvent, state) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCreateWindowEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub border_width: ::std::os::raw::c_int,
    pub override_redirect: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XCreateWindowEvent"][::std::mem::size_of::<XCreateWindowEvent>() - 72usize];
    ["Alignment of XCreateWindowEvent"][::std::mem::align_of::<XCreateWindowEvent>() - 8usize];
    ["Offset of field: XCreateWindowEvent::type_"]
        [::std::mem::offset_of!(XCreateWindowEvent, type_) - 0usize];
    ["Offset of field: XCreateWindowEvent::serial"]
        [::std::mem::offset_of!(XCreateWindowEvent, serial) - 8usize];
    ["Offset of field: XCreateWindowEvent::send_event"]
        [::std::mem::offset_of!(XCreateWindowEvent, send_event) - 16usize];
    ["Offset of field: XCreateWindowEvent::display"]
        [::std::mem::offset_of!(XCreateWindowEvent, display) - 24usize];
    ["Offset of field: XCreateWindowEvent::parent"]
        [::std::mem::offset_of!(XCreateWindowEvent, parent) - 32usize];
    ["Offset of field: XCreateWindowEvent::window"]
        [::std::mem::offset_of!(XCreateWindowEvent, window) - 40usize];
    ["Offset of field: XCreateWindowEvent::x"]
        [::std::mem::offset_of!(XCreateWindowEvent, x) - 48usize];
    ["Offset of field: XCreateWindowEvent::y"]
        [::std::mem::offset_of!(XCreateWindowEvent, y) - 52usize];
    ["Offset of field: XCreateWindowEvent::width"]
        [::std::mem::offset_of!(XCreateWindowEvent, width) - 56usize];
    ["Offset of field: XCreateWindowEvent::height"]
        [::std::mem::offset_of!(XCreateWindowEvent, height) - 60usize];
    ["Offset of field: XCreateWindowEvent::border_width"]
        [::std::mem::offset_of!(XCreateWindowEvent, border_width) - 64usize];
    ["Offset of field: XCreateWindowEvent::override_redirect"]
        [::std::mem::offset_of!(XCreateWindowEvent, override_redirect) - 68usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XDestroyWindowEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XDestroyWindowEvent"][::std::mem::size_of::<XDestroyWindowEvent>() - 48usize];
    ["Alignment of XDestroyWindowEvent"][::std::mem::align_of::<XDestroyWindowEvent>() - 8usize];
    ["Offset of field: XDestroyWindowEvent::type_"]
        [::std::mem::offset_of!(XDestroyWindowEvent, type_) - 0usize];
    ["Offset of field: XDestroyWindowEvent::serial"]
        [::std::mem::offset_of!(XDestroyWindowEvent, serial) - 8usize];
    ["Offset of field: XDestroyWindowEvent::send_event"]
        [::std::mem::offset_of!(XDestroyWindowEvent, send_event) - 16usize];
    ["Offset of field: XDestroyWindowEvent::display"]
        [::std::mem::offset_of!(XDestroyWindowEvent, display) - 24usize];
    ["Offset of field: XDestroyWindowEvent::event"]
        [::std::mem::offset_of!(XDestroyWindowEvent, event) - 32usize];
    ["Offset of field: XDestroyWindowEvent::window"]
        [::std::mem::offset_of!(XDestroyWindowEvent, window) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XUnmapEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XUnmapEvent"][::std::mem::size_of::<XUnmapEvent>() - 56usize];
    ["Alignment of XUnmapEvent"][::std::mem::align_of::<XUnmapEvent>() - 8usize];
    ["Offset of field: XUnmapEvent::type_"][::std::mem::offset_of!(XUnmapEvent, type_) - 0usize];
    ["Offset of field: XUnmapEvent::serial"][::std::mem::offset_of!(XUnmapEvent, serial) - 8usize];
    ["Offset of field: XUnmapEvent::send_event"]
        [::std::mem::offset_of!(XUnmapEvent, send_event) - 16usize];
    ["Offset of field: XUnmapEvent::display"]
        [::std::mem::offset_of!(XUnmapEvent, display) - 24usize];
    ["Offset of field: XUnmapEvent::event"][::std::mem::offset_of!(XUnmapEvent, event) - 32usize];
    ["Offset of field: XUnmapEvent::window"][::std::mem::offset_of!(XUnmapEvent, window) - 40usize];
    ["Offset of field: XUnmapEvent::from_configure"]
        [::std::mem::offset_of!(XUnmapEvent, from_configure) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMapEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XMapEvent"][::std::mem::size_of::<XMapEvent>() - 56usize];
    ["Alignment of XMapEvent"][::std::mem::align_of::<XMapEvent>() - 8usize];
    ["Offset of field: XMapEvent::type_"][::std::mem::offset_of!(XMapEvent, type_) - 0usize];
    ["Offset of field: XMapEvent::serial"][::std::mem::offset_of!(XMapEvent, serial) - 8usize];
    ["Offset of field: XMapEvent::send_event"]
        [::std::mem::offset_of!(XMapEvent, send_event) - 16usize];
    ["Offset of field: XMapEvent::display"][::std::mem::offset_of!(XMapEvent, display) - 24usize];
    ["Offset of field: XMapEvent::event"][::std::mem::offset_of!(XMapEvent, event) - 32usize];
    ["Offset of field: XMapEvent::window"][::std::mem::offset_of!(XMapEvent, window) - 40usize];
    ["Offset of field: XMapEvent::override_redirect"]
        [::std::mem::offset_of!(XMapEvent, override_redirect) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMapRequestEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XMapRequestEvent"][::std::mem::size_of::<XMapRequestEvent>() - 48usize];
    ["Alignment of XMapRequestEvent"][::std::mem::align_of::<XMapRequestEvent>() - 8usize];
    ["Offset of field: XMapRequestEvent::type_"]
        [::std::mem::offset_of!(XMapRequestEvent, type_) - 0usize];
    ["Offset of field: XMapRequestEvent::serial"]
        [::std::mem::offset_of!(XMapRequestEvent, serial) - 8usize];
    ["Offset of field: XMapRequestEvent::send_event"]
        [::std::mem::offset_of!(XMapRequestEvent, send_event) - 16usize];
    ["Offset of field: XMapRequestEvent::display"]
        [::std::mem::offset_of!(XMapRequestEvent, display) - 24usize];
    ["Offset of field: XMapRequestEvent::parent"]
        [::std::mem::offset_of!(XMapRequestEvent, parent) - 32usize];
    ["Offset of field: XMapRequestEvent::window"]
        [::std::mem::offset_of!(XMapRequestEvent, window) - 40usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XReparentEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub override_redirect: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XReparentEvent"][::std::mem::size_of::<XReparentEvent>() - 72usize];
    ["Alignment of XReparentEvent"][::std::mem::align_of::<XReparentEvent>() - 8usize];
    ["Offset of field: XReparentEvent::type_"]
        [::std::mem::offset_of!(XReparentEvent, type_) - 0usize];
    ["Offset of field: XReparentEvent::serial"]
        [::std::mem::offset_of!(XReparentEvent, serial) - 8usize];
    ["Offset of field: XReparentEvent::send_event"]
        [::std::mem::offset_of!(XReparentEvent, send_event) - 16usize];
    ["Offset of field: XReparentEvent::display"]
        [::std::mem::offset_of!(XReparentEvent, display) - 24usize];
    ["Offset of field: XReparentEvent::event"]
        [::std::mem::offset_of!(XReparentEvent, event) - 32usize];
    ["Offset of field: XReparentEvent::window"]
        [::std::mem::offset_of!(XReparentEvent, window) - 40usize];
    ["Offset of field: XReparentEvent::parent"]
        [::std::mem::offset_of!(XReparentEvent, parent) - 48usize];
    ["Offset of field: XReparentEvent::x"][::std::mem::offset_of!(XReparentEvent, x) - 56usize];
    ["Offset of field: XReparentEvent::y"][::std::mem::offset_of!(XReparentEvent, y) - 60usize];
    ["Offset of field: XReparentEvent::override_redirect"]
        [::std::mem::offset_of!(XReparentEvent, override_redirect) - 64usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XConfigureEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub border_width: ::std::os::raw::c_int,
    pub above: Window,
    pub override_redirect: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XConfigureEvent"][::std::mem::size_of::<XConfigureEvent>() - 88usize];
    ["Alignment of XConfigureEvent"][::std::mem::align_of::<XConfigureEvent>() - 8usize];
    ["Offset of field: XConfigureEvent::type_"]
        [::std::mem::offset_of!(XConfigureEvent, type_) - 0usize];
    ["Offset of field: XConfigureEvent::serial"]
        [::std::mem::offset_of!(XConfigureEvent, serial) - 8usize];
    ["Offset of field: XConfigureEvent::send_event"]
        [::std::mem::offset_of!(XConfigureEvent, send_event) - 16usize];
    ["Offset of field: XConfigureEvent::display"]
        [::std::mem::offset_of!(XConfigureEvent, display) - 24usize];
    ["Offset of field: XConfigureEvent::event"]
        [::std::mem::offset_of!(XConfigureEvent, event) - 32usize];
    ["Offset of field: XConfigureEvent::window"]
        [::std::mem::offset_of!(XConfigureEvent, window) - 40usize];
    ["Offset of field: XConfigureEvent::x"][::std::mem::offset_of!(XConfigureEvent, x) - 48usize];
    ["Offset of field: XConfigureEvent::y"][::std::mem::offset_of!(XConfigureEvent, y) - 52usize];
    ["Offset of field: XConfigureEvent::width"]
        [::std::mem::offset_of!(XConfigureEvent, width) - 56usize];
    ["Offset of field: XConfigureEvent::height"]
        [::std::mem::offset_of!(XConfigureEvent, height) - 60usize];
    ["Offset of field: XConfigureEvent::border_width"]
        [::std::mem::offset_of!(XConfigureEvent, border_width) - 64usize];
    ["Offset of field: XConfigureEvent::above"]
        [::std::mem::offset_of!(XConfigureEvent, above) - 72usize];
    ["Offset of field: XConfigureEvent::override_redirect"]
        [::std::mem::offset_of!(XConfigureEvent, override_redirect) - 80usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGravityEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XGravityEvent"][::std::mem::size_of::<XGravityEvent>() - 56usize];
    ["Alignment of XGravityEvent"][::std::mem::align_of::<XGravityEvent>() - 8usize];
    ["Offset of field: XGravityEvent::type_"]
        [::std::mem::offset_of!(XGravityEvent, type_) - 0usize];
    ["Offset of field: XGravityEvent::serial"]
        [::std::mem::offset_of!(XGravityEvent, serial) - 8usize];
    ["Offset of field: XGravityEvent::send_event"]
        [::std::mem::offset_of!(XGravityEvent, send_event) - 16usize];
    ["Offset of field: XGravityEvent::display"]
        [::std::mem::offset_of!(XGravityEvent, display) - 24usize];
    ["Offset of field: XGravityEvent::event"]
        [::std::mem::offset_of!(XGravityEvent, event) - 32usize];
    ["Offset of field: XGravityEvent::window"]
        [::std::mem::offset_of!(XGravityEvent, window) - 40usize];
    ["Offset of field: XGravityEvent::x"][::std::mem::offset_of!(XGravityEvent, x) - 48usize];
    ["Offset of field: XGravityEvent::y"][::std::mem::offset_of!(XGravityEvent, y) - 52usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XResizeRequestEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XResizeRequestEvent"][::std::mem::size_of::<XResizeRequestEvent>() - 48usize];
    ["Alignment of XResizeRequestEvent"][::std::mem::align_of::<XResizeRequestEvent>() - 8usize];
    ["Offset of field: XResizeRequestEvent::type_"]
        [::std::mem::offset_of!(XResizeRequestEvent, type_) - 0usize];
    ["Offset of field: XResizeRequestEvent::serial"]
        [::std::mem::offset_of!(XResizeRequestEvent, serial) - 8usize];
    ["Offset of field: XResizeRequestEvent::send_event"]
        [::std::mem::offset_of!(XResizeRequestEvent, send_event) - 16usize];
    ["Offset of field: XResizeRequestEvent::display"]
        [::std::mem::offset_of!(XResizeRequestEvent, display) - 24usize];
    ["Offset of field: XResizeRequestEvent::window"]
        [::std::mem::offset_of!(XResizeRequestEvent, window) - 32usize];
    ["Offset of field: XResizeRequestEvent::width"]
        [::std::mem::offset_of!(XResizeRequestEvent, width) - 40usize];
    ["Offset of field: XResizeRequestEvent::height"]
        [::std::mem::offset_of!(XResizeRequestEvent, height) - 44usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XConfigureRequestEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub border_width: ::std::os::raw::c_int,
    pub above: Window,
    pub detail: ::std::os::raw::c_int,
    pub value_mask: ::std::os::raw::c_ulong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XConfigureRequestEvent"][::std::mem::size_of::<XConfigureRequestEvent>() - 96usize];
    ["Alignment of XConfigureRequestEvent"]
        [::std::mem::align_of::<XConfigureRequestEvent>() - 8usize];
    ["Offset of field: XConfigureRequestEvent::type_"]
        [::std::mem::offset_of!(XConfigureRequestEvent, type_) - 0usize];
    ["Offset of field: XConfigureRequestEvent::serial"]
        [::std::mem::offset_of!(XConfigureRequestEvent, serial) - 8usize];
    ["Offset of field: XConfigureRequestEvent::send_event"]
        [::std::mem::offset_of!(XConfigureRequestEvent, send_event) - 16usize];
    ["Offset of field: XConfigureRequestEvent::display"]
        [::std::mem::offset_of!(XConfigureRequestEvent, display) - 24usize];
    ["Offset of field: XConfigureRequestEvent::parent"]
        [::std::mem::offset_of!(XConfigureRequestEvent, parent) - 32usize];
    ["Offset of field: XConfigureRequestEvent::window"]
        [::std::mem::offset_of!(XConfigureRequestEvent, window) - 40usize];
    ["Offset of field: XConfigureRequestEvent::x"]
        [::std::mem::offset_of!(XConfigureRequestEvent, x) - 48usize];
    ["Offset of field: XConfigureRequestEvent::y"]
        [::std::mem::offset_of!(XConfigureRequestEvent, y) - 52usize];
    ["Offset of field: XConfigureRequestEvent::width"]
        [::std::mem::offset_of!(XConfigureRequestEvent, width) - 56usize];
    ["Offset of field: XConfigureRequestEvent::height"]
        [::std::mem::offset_of!(XConfigureRequestEvent, height) - 60usize];
    ["Offset of field: XConfigureRequestEvent::border_width"]
        [::std::mem::offset_of!(XConfigureRequestEvent, border_width) - 64usize];
    ["Offset of field: XConfigureRequestEvent::above"]
        [::std::mem::offset_of!(XConfigureRequestEvent, above) - 72usize];
    ["Offset of field: XConfigureRequestEvent::detail"]
        [::std::mem::offset_of!(XConfigureRequestEvent, detail) - 80usize];
    ["Offset of field: XConfigureRequestEvent::value_mask"]
        [::std::mem::offset_of!(XConfigureRequestEvent, value_mask) - 88usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCirculateEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XCirculateEvent"][::std::mem::size_of::<XCirculateEvent>() - 56usize];
    ["Alignment of XCirculateEvent"][::std::mem::align_of::<XCirculateEvent>() - 8usize];
    ["Offset of field: XCirculateEvent::type_"]
        [::std::mem::offset_of!(XCirculateEvent, type_) - 0usize];
    ["Offset of field: XCirculateEvent::serial"]
        [::std::mem::offset_of!(XCirculateEvent, serial) - 8usize];
    ["Offset of field: XCirculateEvent::send_event"]
        [::std::mem::offset_of!(XCirculateEvent, send_event) - 16usize];
    ["Offset of field: XCirculateEvent::display"]
        [::std::mem::offset_of!(XCirculateEvent, display) - 24usize];
    ["Offset of field: XCirculateEvent::event"]
        [::std::mem::offset_of!(XCirculateEvent, event) - 32usize];
    ["Offset of field: XCirculateEvent::window"]
        [::std::mem::offset_of!(XCirculateEvent, window) - 40usize];
    ["Offset of field: XCirculateEvent::place"]
        [::std::mem::offset_of!(XCirculateEvent, place) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCirculateRequestEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XCirculateRequestEvent"][::std::mem::size_of::<XCirculateRequestEvent>() - 56usize];
    ["Alignment of XCirculateRequestEvent"]
        [::std::mem::align_of::<XCirculateRequestEvent>() - 8usize];
    ["Offset of field: XCirculateRequestEvent::type_"]
        [::std::mem::offset_of!(XCirculateRequestEvent, type_) - 0usize];
    ["Offset of field: XCirculateRequestEvent::serial"]
        [::std::mem::offset_of!(XCirculateRequestEvent, serial) - 8usize];
    ["Offset of field: XCirculateRequestEvent::send_event"]
        [::std::mem::offset_of!(XCirculateRequestEvent, send_event) - 16usize];
    ["Offset of field: XCirculateRequestEvent::display"]
        [::std::mem::offset_of!(XCirculateRequestEvent, display) - 24usize];
    ["Offset of field: XCirculateRequestEvent::parent"]
        [::std::mem::offset_of!(XCirculateRequestEvent, parent) - 32usize];
    ["Offset of field: XCirculateRequestEvent::window"]
        [::std::mem::offset_of!(XCirculateRequestEvent, window) - 40usize];
    ["Offset of field: XCirculateRequestEvent::place"]
        [::std::mem::offset_of!(XCirculateRequestEvent, place) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPropertyEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XPropertyEvent"][::std::mem::size_of::<XPropertyEvent>() - 64usize];
    ["Alignment of XPropertyEvent"][::std::mem::align_of::<XPropertyEvent>() - 8usize];
    ["Offset of field: XPropertyEvent::type_"]
        [::std::mem::offset_of!(XPropertyEvent, type_) - 0usize];
    ["Offset of field: XPropertyEvent::serial"]
        [::std::mem::offset_of!(XPropertyEvent, serial) - 8usize];
    ["Offset of field: XPropertyEvent::send_event"]
        [::std::mem::offset_of!(XPropertyEvent, send_event) - 16usize];
    ["Offset of field: XPropertyEvent::display"]
        [::std::mem::offset_of!(XPropertyEvent, display) - 24usize];
    ["Offset of field: XPropertyEvent::window"]
        [::std::mem::offset_of!(XPropertyEvent, window) - 32usize];
    ["Offset of field: XPropertyEvent::atom"]
        [::std::mem::offset_of!(XPropertyEvent, atom) - 40usize];
    ["Offset of field: XPropertyEvent::time"]
        [::std::mem::offset_of!(XPropertyEvent, time) - 48usize];
    ["Offset of field: XPropertyEvent::state"]
        [::std::mem::offset_of!(XPropertyEvent, state) - 56usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionClearEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XSelectionClearEvent"][::std::mem::size_of::<XSelectionClearEvent>() - 56usize];
    ["Alignment of XSelectionClearEvent"][::std::mem::align_of::<XSelectionClearEvent>() - 8usize];
    ["Offset of field: XSelectionClearEvent::type_"]
        [::std::mem::offset_of!(XSelectionClearEvent, type_) - 0usize];
    ["Offset of field: XSelectionClearEvent::serial"]
        [::std::mem::offset_of!(XSelectionClearEvent, serial) - 8usize];
    ["Offset of field: XSelectionClearEvent::send_event"]
        [::std::mem::offset_of!(XSelectionClearEvent, send_event) - 16usize];
    ["Offset of field: XSelectionClearEvent::display"]
        [::std::mem::offset_of!(XSelectionClearEvent, display) - 24usize];
    ["Offset of field: XSelectionClearEvent::window"]
        [::std::mem::offset_of!(XSelectionClearEvent, window) - 32usize];
    ["Offset of field: XSelectionClearEvent::selection"]
        [::std::mem::offset_of!(XSelectionClearEvent, selection) - 40usize];
    ["Offset of field: XSelectionClearEvent::time"]
        [::std::mem::offset_of!(XSelectionClearEvent, time) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionRequestEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XSelectionRequestEvent"][::std::mem::size_of::<XSelectionRequestEvent>() - 80usize];
    ["Alignment of XSelectionRequestEvent"]
        [::std::mem::align_of::<XSelectionRequestEvent>() - 8usize];
    ["Offset of field: XSelectionRequestEvent::type_"]
        [::std::mem::offset_of!(XSelectionRequestEvent, type_) - 0usize];
    ["Offset of field: XSelectionRequestEvent::serial"]
        [::std::mem::offset_of!(XSelectionRequestEvent, serial) - 8usize];
    ["Offset of field: XSelectionRequestEvent::send_event"]
        [::std::mem::offset_of!(XSelectionRequestEvent, send_event) - 16usize];
    ["Offset of field: XSelectionRequestEvent::display"]
        [::std::mem::offset_of!(XSelectionRequestEvent, display) - 24usize];
    ["Offset of field: XSelectionRequestEvent::owner"]
        [::std::mem::offset_of!(XSelectionRequestEvent, owner) - 32usize];
    ["Offset of field: XSelectionRequestEvent::requestor"]
        [::std::mem::offset_of!(XSelectionRequestEvent, requestor) - 40usize];
    ["Offset of field: XSelectionRequestEvent::selection"]
        [::std::mem::offset_of!(XSelectionRequestEvent, selection) - 48usize];
    ["Offset of field: XSelectionRequestEvent::target"]
        [::std::mem::offset_of!(XSelectionRequestEvent, target) - 56usize];
    ["Offset of field: XSelectionRequestEvent::property"]
        [::std::mem::offset_of!(XSelectionRequestEvent, property) - 64usize];
    ["Offset of field: XSelectionRequestEvent::time"]
        [::std::mem::offset_of!(XSelectionRequestEvent, time) - 72usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSelectionEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XSelectionEvent"][::std::mem::size_of::<XSelectionEvent>() - 72usize];
    ["Alignment of XSelectionEvent"][::std::mem::align_of::<XSelectionEvent>() - 8usize];
    ["Offset of field: XSelectionEvent::type_"]
        [::std::mem::offset_of!(XSelectionEvent, type_) - 0usize];
    ["Offset of field: XSelectionEvent::serial"]
        [::std::mem::offset_of!(XSelectionEvent, serial) - 8usize];
    ["Offset of field: XSelectionEvent::send_event"]
        [::std::mem::offset_of!(XSelectionEvent, send_event) - 16usize];
    ["Offset of field: XSelectionEvent::display"]
        [::std::mem::offset_of!(XSelectionEvent, display) - 24usize];
    ["Offset of field: XSelectionEvent::requestor"]
        [::std::mem::offset_of!(XSelectionEvent, requestor) - 32usize];
    ["Offset of field: XSelectionEvent::selection"]
        [::std::mem::offset_of!(XSelectionEvent, selection) - 40usize];
    ["Offset of field: XSelectionEvent::target"]
        [::std::mem::offset_of!(XSelectionEvent, target) - 48usize];
    ["Offset of field: XSelectionEvent::property"]
        [::std::mem::offset_of!(XSelectionEvent, property) - 56usize];
    ["Offset of field: XSelectionEvent::time"]
        [::std::mem::offset_of!(XSelectionEvent, time) - 64usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XColormapEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XColormapEvent"][::std::mem::size_of::<XColormapEvent>() - 56usize];
    ["Alignment of XColormapEvent"][::std::mem::align_of::<XColormapEvent>() - 8usize];
    ["Offset of field: XColormapEvent::type_"]
        [::std::mem::offset_of!(XColormapEvent, type_) - 0usize];
    ["Offset of field: XColormapEvent::serial"]
        [::std::mem::offset_of!(XColormapEvent, serial) - 8usize];
    ["Offset of field: XColormapEvent::send_event"]
        [::std::mem::offset_of!(XColormapEvent, send_event) - 16usize];
    ["Offset of field: XColormapEvent::display"]
        [::std::mem::offset_of!(XColormapEvent, display) - 24usize];
    ["Offset of field: XColormapEvent::window"]
        [::std::mem::offset_of!(XColormapEvent, window) - 32usize];
    ["Offset of field: XColormapEvent::colormap"]
        [::std::mem::offset_of!(XColormapEvent, colormap) - 40usize];
    ["Offset of field: XColormapEvent::new"][::std::mem::offset_of!(XColormapEvent, new) - 48usize];
    ["Offset of field: XColormapEvent::state"]
        [::std::mem::offset_of!(XColormapEvent, state) - 52usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XClientMessageEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: ::std::os::raw::c_int,
    pub data: XClientMessageEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union XClientMessageEvent__bindgen_ty_1 {
    pub b: [::std::os::raw::c_char; 20usize],
    pub s: [::std::os::raw::c_short; 10usize],
    pub l: [::std::os::raw::c_long; 5usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XClientMessageEvent__bindgen_ty_1"]
        [::std::mem::size_of::<XClientMessageEvent__bindgen_ty_1>() - 40usize];
    ["Alignment of XClientMessageEvent__bindgen_ty_1"]
        [::std::mem::align_of::<XClientMessageEvent__bindgen_ty_1>() - 8usize];
    ["Offset of field: XClientMessageEvent__bindgen_ty_1::b"]
        [::std::mem::offset_of!(XClientMessageEvent__bindgen_ty_1, b) - 0usize];
    ["Offset of field: XClientMessageEvent__bindgen_ty_1::s"]
        [::std::mem::offset_of!(XClientMessageEvent__bindgen_ty_1, s) - 0usize];
    ["Offset of field: XClientMessageEvent__bindgen_ty_1::l"]
        [::std::mem::offset_of!(XClientMessageEvent__bindgen_ty_1, l) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XClientMessageEvent"][::std::mem::size_of::<XClientMessageEvent>() - 96usize];
    ["Alignment of XClientMessageEvent"][::std::mem::align_of::<XClientMessageEvent>() - 8usize];
    ["Offset of field: XClientMessageEvent::type_"]
        [::std::mem::offset_of!(XClientMessageEvent, type_) - 0usize];
    ["Offset of field: XClientMessageEvent::serial"]
        [::std::mem::offset_of!(XClientMessageEvent, serial) - 8usize];
    ["Offset of field: XClientMessageEvent::send_event"]
        [::std::mem::offset_of!(XClientMessageEvent, send_event) - 16usize];
    ["Offset of field: XClientMessageEvent::display"]
        [::std::mem::offset_of!(XClientMessageEvent, display) - 24usize];
    ["Offset of field: XClientMessageEvent::window"]
        [::std::mem::offset_of!(XClientMessageEvent, window) - 32usize];
    ["Offset of field: XClientMessageEvent::message_type"]
        [::std::mem::offset_of!(XClientMessageEvent, message_type) - 40usize];
    ["Offset of field: XClientMessageEvent::format"]
        [::std::mem::offset_of!(XClientMessageEvent, format) - 48usize];
    ["Offset of field: XClientMessageEvent::data"]
        [::std::mem::offset_of!(XClientMessageEvent, data) - 56usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XMappingEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: ::std::os::raw::c_int,
    pub first_keycode: ::std::os::raw::c_int,
    pub count: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XMappingEvent"][::std::mem::size_of::<XMappingEvent>() - 56usize];
    ["Alignment of XMappingEvent"][::std::mem::align_of::<XMappingEvent>() - 8usize];
    ["Offset of field: XMappingEvent::type_"]
        [::std::mem::offset_of!(XMappingEvent, type_) - 0usize];
    ["Offset of field: XMappingEvent::serial"]
        [::std::mem::offset_of!(XMappingEvent, serial) - 8usize];
    ["Offset of field: XMappingEvent::send_event"]
        [::std::mem::offset_of!(XMappingEvent, send_event) - 16usize];
    ["Offset of field: XMappingEvent::display"]
        [::std::mem::offset_of!(XMappingEvent, display) - 24usize];
    ["Offset of field: XMappingEvent::window"]
        [::std::mem::offset_of!(XMappingEvent, window) - 32usize];
    ["Offset of field: XMappingEvent::request"]
        [::std::mem::offset_of!(XMappingEvent, request) - 40usize];
    ["Offset of field: XMappingEvent::first_keycode"]
        [::std::mem::offset_of!(XMappingEvent, first_keycode) - 44usize];
    ["Offset of field: XMappingEvent::count"]
        [::std::mem::offset_of!(XMappingEvent, count) - 48usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XErrorEvent {
    pub type_: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: ::std::os::raw::c_ulong,
    pub error_code: ::std::os::raw::c_uchar,
    pub request_code: ::std::os::raw::c_uchar,
    pub minor_code: ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XErrorEvent"][::std::mem::size_of::<XErrorEvent>() - 40usize];
    ["Alignment of XErrorEvent"][::std::mem::align_of::<XErrorEvent>() - 8usize];
    ["Offset of field: XErrorEvent::type_"][::std::mem::offset_of!(XErrorEvent, type_) - 0usize];
    ["Offset of field: XErrorEvent::display"]
        [::std::mem::offset_of!(XErrorEvent, display) - 8usize];
    ["Offset of field: XErrorEvent::resourceid"]
        [::std::mem::offset_of!(XErrorEvent, resourceid) - 16usize];
    ["Offset of field: XErrorEvent::serial"][::std::mem::offset_of!(XErrorEvent, serial) - 24usize];
    ["Offset of field: XErrorEvent::error_code"]
        [::std::mem::offset_of!(XErrorEvent, error_code) - 32usize];
    ["Offset of field: XErrorEvent::request_code"]
        [::std::mem::offset_of!(XErrorEvent, request_code) - 33usize];
    ["Offset of field: XErrorEvent::minor_code"]
        [::std::mem::offset_of!(XErrorEvent, minor_code) - 34usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XAnyEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XAnyEvent"][::std::mem::size_of::<XAnyEvent>() - 40usize];
    ["Alignment of XAnyEvent"][::std::mem::align_of::<XAnyEvent>() - 8usize];
    ["Offset of field: XAnyEvent::type_"][::std::mem::offset_of!(XAnyEvent, type_) - 0usize];
    ["Offset of field: XAnyEvent::serial"][::std::mem::offset_of!(XAnyEvent, serial) - 8usize];
    ["Offset of field: XAnyEvent::send_event"]
        [::std::mem::offset_of!(XAnyEvent, send_event) - 16usize];
    ["Offset of field: XAnyEvent::display"][::std::mem::offset_of!(XAnyEvent, display) - 24usize];
    ["Offset of field: XAnyEvent::window"][::std::mem::offset_of!(XAnyEvent, window) - 32usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGenericEvent {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub extension: ::std::os::raw::c_int,
    pub evtype: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XGenericEvent"][::std::mem::size_of::<XGenericEvent>() - 40usize];
    ["Alignment of XGenericEvent"][::std::mem::align_of::<XGenericEvent>() - 8usize];
    ["Offset of field: XGenericEvent::type_"]
        [::std::mem::offset_of!(XGenericEvent, type_) - 0usize];
    ["Offset of field: XGenericEvent::serial"]
        [::std::mem::offset_of!(XGenericEvent, serial) - 8usize];
    ["Offset of field: XGenericEvent::send_event"]
        [::std::mem::offset_of!(XGenericEvent, send_event) - 16usize];
    ["Offset of field: XGenericEvent::display"]
        [::std::mem::offset_of!(XGenericEvent, display) - 24usize];
    ["Offset of field: XGenericEvent::extension"]
        [::std::mem::offset_of!(XGenericEvent, extension) - 32usize];
    ["Offset of field: XGenericEvent::evtype"]
        [::std::mem::offset_of!(XGenericEvent, evtype) - 36usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XGenericEventCookie {
    pub type_: ::std::os::raw::c_int,
    pub serial: ::std::os::raw::c_ulong,
    pub send_event: ::std::os::raw::c_int,
    pub display: *mut Display,
    pub extension: ::std::os::raw::c_int,
    pub evtype: ::std::os::raw::c_int,
    pub cookie: ::std::os::raw::c_uint,
    pub data: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XGenericEventCookie"][::std::mem::size_of::<XGenericEventCookie>() - 56usize];
    ["Alignment of XGenericEventCookie"][::std::mem::align_of::<XGenericEventCookie>() - 8usize];
    ["Offset of field: XGenericEventCookie::type_"]
        [::std::mem::offset_of!(XGenericEventCookie, type_) - 0usize];
    ["Offset of field: XGenericEventCookie::serial"]
        [::std::mem::offset_of!(XGenericEventCookie, serial) - 8usize];
    ["Offset of field: XGenericEventCookie::send_event"]
        [::std::mem::offset_of!(XGenericEventCookie, send_event) - 16usize];
    ["Offset of field: XGenericEventCookie::display"]
        [::std::mem::offset_of!(XGenericEventCookie, display) - 24usize];
    ["Offset of field: XGenericEventCookie::extension"]
        [::std::mem::offset_of!(XGenericEventCookie, extension) - 32usize];
    ["Offset of field: XGenericEventCookie::evtype"]
        [::std::mem::offset_of!(XGenericEventCookie, evtype) - 36usize];
    ["Offset of field: XGenericEventCookie::cookie"]
        [::std::mem::offset_of!(XGenericEventCookie, cookie) - 40usize];
    ["Offset of field: XGenericEventCookie::data"]
        [::std::mem::offset_of!(XGenericEventCookie, data) - 48usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XEvent {
    pub type_: ::std::os::raw::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [::std::os::raw::c_long; 24usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XEvent"][::std::mem::size_of::<_XEvent>() - 192usize];
    ["Alignment of _XEvent"][::std::mem::align_of::<_XEvent>() - 8usize];
    ["Offset of field: _XEvent::type_"][::std::mem::offset_of!(_XEvent, type_) - 0usize];
    ["Offset of field: _XEvent::xany"][::std::mem::offset_of!(_XEvent, xany) - 0usize];
    ["Offset of field: _XEvent::xkey"][::std::mem::offset_of!(_XEvent, xkey) - 0usize];
    ["Offset of field: _XEvent::xbutton"][::std::mem::offset_of!(_XEvent, xbutton) - 0usize];
    ["Offset of field: _XEvent::xmotion"][::std::mem::offset_of!(_XEvent, xmotion) - 0usize];
    ["Offset of field: _XEvent::xcrossing"][::std::mem::offset_of!(_XEvent, xcrossing) - 0usize];
    ["Offset of field: _XEvent::xfocus"][::std::mem::offset_of!(_XEvent, xfocus) - 0usize];
    ["Offset of field: _XEvent::xexpose"][::std::mem::offset_of!(_XEvent, xexpose) - 0usize];
    ["Offset of field: _XEvent::xgraphicsexpose"]
        [::std::mem::offset_of!(_XEvent, xgraphicsexpose) - 0usize];
    ["Offset of field: _XEvent::xnoexpose"][::std::mem::offset_of!(_XEvent, xnoexpose) - 0usize];
    ["Offset of field: _XEvent::xvisibility"]
        [::std::mem::offset_of!(_XEvent, xvisibility) - 0usize];
    ["Offset of field: _XEvent::xcreatewindow"]
        [::std::mem::offset_of!(_XEvent, xcreatewindow) - 0usize];
    ["Offset of field: _XEvent::xdestroywindow"]
        [::std::mem::offset_of!(_XEvent, xdestroywindow) - 0usize];
    ["Offset of field: _XEvent::xunmap"][::std::mem::offset_of!(_XEvent, xunmap) - 0usize];
    ["Offset of field: _XEvent::xmap"][::std::mem::offset_of!(_XEvent, xmap) - 0usize];
    ["Offset of field: _XEvent::xmaprequest"]
        [::std::mem::offset_of!(_XEvent, xmaprequest) - 0usize];
    ["Offset of field: _XEvent::xreparent"][::std::mem::offset_of!(_XEvent, xreparent) - 0usize];
    ["Offset of field: _XEvent::xconfigure"][::std::mem::offset_of!(_XEvent, xconfigure) - 0usize];
    ["Offset of field: _XEvent::xgravity"][::std::mem::offset_of!(_XEvent, xgravity) - 0usize];
    ["Offset of field: _XEvent::xresizerequest"]
        [::std::mem::offset_of!(_XEvent, xresizerequest) - 0usize];
    ["Offset of field: _XEvent::xconfigurerequest"]
        [::std::mem::offset_of!(_XEvent, xconfigurerequest) - 0usize];
    ["Offset of field: _XEvent::xcirculate"][::std::mem::offset_of!(_XEvent, xcirculate) - 0usize];
    ["Offset of field: _XEvent::xcirculaterequest"]
        [::std::mem::offset_of!(_XEvent, xcirculaterequest) - 0usize];
    ["Offset of field: _XEvent::xproperty"][::std::mem::offset_of!(_XEvent, xproperty) - 0usize];
    ["Offset of field: _XEvent::xselectionclear"]
        [::std::mem::offset_of!(_XEvent, xselectionclear) - 0usize];
    ["Offset of field: _XEvent::xselectionrequest"]
        [::std::mem::offset_of!(_XEvent, xselectionrequest) - 0usize];
    ["Offset of field: _XEvent::xselection"][::std::mem::offset_of!(_XEvent, xselection) - 0usize];
    ["Offset of field: _XEvent::xcolormap"][::std::mem::offset_of!(_XEvent, xcolormap) - 0usize];
    ["Offset of field: _XEvent::xclient"][::std::mem::offset_of!(_XEvent, xclient) - 0usize];
    ["Offset of field: _XEvent::xmapping"][::std::mem::offset_of!(_XEvent, xmapping) - 0usize];
    ["Offset of field: _XEvent::xerror"][::std::mem::offset_of!(_XEvent, xerror) - 0usize];
    ["Offset of field: _XEvent::xkeymap"][::std::mem::offset_of!(_XEvent, xkeymap) - 0usize];
    ["Offset of field: _XEvent::xgeneric"][::std::mem::offset_of!(_XEvent, xgeneric) - 0usize];
    ["Offset of field: _XEvent::xcookie"][::std::mem::offset_of!(_XEvent, xcookie) - 0usize];
    ["Offset of field: _XEvent::pad"][::std::mem::offset_of!(_XEvent, pad) - 0usize];
};
pub type XEvent = _XEvent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XCharStruct {
    pub lbearing: ::std::os::raw::c_short,
    pub rbearing: ::std::os::raw::c_short,
    pub width: ::std::os::raw::c_short,
    pub ascent: ::std::os::raw::c_short,
    pub descent: ::std::os::raw::c_short,
    pub attributes: ::std::os::raw::c_ushort,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XCharStruct"][::std::mem::size_of::<XCharStruct>() - 12usize];
    ["Alignment of XCharStruct"][::std::mem::align_of::<XCharStruct>() - 2usize];
    ["Offset of field: XCharStruct::lbearing"]
        [::std::mem::offset_of!(XCharStruct, lbearing) - 0usize];
    ["Offset of field: XCharStruct::rbearing"]
        [::std::mem::offset_of!(XCharStruct, rbearing) - 2usize];
    ["Offset of field: XCharStruct::width"][::std::mem::offset_of!(XCharStruct, width) - 4usize];
    ["Offset of field: XCharStruct::ascent"][::std::mem::offset_of!(XCharStruct, ascent) - 6usize];
    ["Offset of field: XCharStruct::descent"]
        [::std::mem::offset_of!(XCharStruct, descent) - 8usize];
    ["Offset of field: XCharStruct::attributes"]
        [::std::mem::offset_of!(XCharStruct, attributes) - 10usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFontProp {
    pub name: Atom,
    pub card32: ::std::os::raw::c_ulong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XFontProp"][::std::mem::size_of::<XFontProp>() - 16usize];
    ["Alignment of XFontProp"][::std::mem::align_of::<XFontProp>() - 8usize];
    ["Offset of field: XFontProp::name"][::std::mem::offset_of!(XFontProp, name) - 0usize];
    ["Offset of field: XFontProp::card32"][::std::mem::offset_of!(XFontProp, card32) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFontStruct {
    pub ext_data: *mut XExtData,
    pub fid: Font,
    pub direction: ::std::os::raw::c_uint,
    pub min_char_or_byte2: ::std::os::raw::c_uint,
    pub max_char_or_byte2: ::std::os::raw::c_uint,
    pub min_byte1: ::std::os::raw::c_uint,
    pub max_byte1: ::std::os::raw::c_uint,
    pub all_chars_exist: ::std::os::raw::c_int,
    pub default_char: ::std::os::raw::c_uint,
    pub n_properties: ::std::os::raw::c_int,
    pub properties: *mut XFontProp,
    pub min_bounds: XCharStruct,
    pub max_bounds: XCharStruct,
    pub per_char: *mut XCharStruct,
    pub ascent: ::std::os::raw::c_int,
    pub descent: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XFontStruct"][::std::mem::size_of::<XFontStruct>() - 96usize];
    ["Alignment of XFontStruct"][::std::mem::align_of::<XFontStruct>() - 8usize];
    ["Offset of field: XFontStruct::ext_data"]
        [::std::mem::offset_of!(XFontStruct, ext_data) - 0usize];
    ["Offset of field: XFontStruct::fid"][::std::mem::offset_of!(XFontStruct, fid) - 8usize];
    ["Offset of field: XFontStruct::direction"]
        [::std::mem::offset_of!(XFontStruct, direction) - 16usize];
    ["Offset of field: XFontStruct::min_char_or_byte2"]
        [::std::mem::offset_of!(XFontStruct, min_char_or_byte2) - 20usize];
    ["Offset of field: XFontStruct::max_char_or_byte2"]
        [::std::mem::offset_of!(XFontStruct, max_char_or_byte2) - 24usize];
    ["Offset of field: XFontStruct::min_byte1"]
        [::std::mem::offset_of!(XFontStruct, min_byte1) - 28usize];
    ["Offset of field: XFontStruct::max_byte1"]
        [::std::mem::offset_of!(XFontStruct, max_byte1) - 32usize];
    ["Offset of field: XFontStruct::all_chars_exist"]
        [::std::mem::offset_of!(XFontStruct, all_chars_exist) - 36usize];
    ["Offset of field: XFontStruct::default_char"]
        [::std::mem::offset_of!(XFontStruct, default_char) - 40usize];
    ["Offset of field: XFontStruct::n_properties"]
        [::std::mem::offset_of!(XFontStruct, n_properties) - 44usize];
    ["Offset of field: XFontStruct::properties"]
        [::std::mem::offset_of!(XFontStruct, properties) - 48usize];
    ["Offset of field: XFontStruct::min_bounds"]
        [::std::mem::offset_of!(XFontStruct, min_bounds) - 56usize];
    ["Offset of field: XFontStruct::max_bounds"]
        [::std::mem::offset_of!(XFontStruct, max_bounds) - 68usize];
    ["Offset of field: XFontStruct::per_char"]
        [::std::mem::offset_of!(XFontStruct, per_char) - 80usize];
    ["Offset of field: XFontStruct::ascent"][::std::mem::offset_of!(XFontStruct, ascent) - 88usize];
    ["Offset of field: XFontStruct::descent"]
        [::std::mem::offset_of!(XFontStruct, descent) - 92usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XTextItem {
    pub chars: *mut ::std::os::raw::c_char,
    pub nchars: ::std::os::raw::c_int,
    pub delta: ::std::os::raw::c_int,
    pub font: Font,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XTextItem"][::std::mem::size_of::<XTextItem>() - 24usize];
    ["Alignment of XTextItem"][::std::mem::align_of::<XTextItem>() - 8usize];
    ["Offset of field: XTextItem::chars"][::std::mem::offset_of!(XTextItem, chars) - 0usize];
    ["Offset of field: XTextItem::nchars"][::std::mem::offset_of!(XTextItem, nchars) - 8usize];
    ["Offset of field: XTextItem::delta"][::std::mem::offset_of!(XTextItem, delta) - 12usize];
    ["Offset of field: XTextItem::font"][::std::mem::offset_of!(XTextItem, font) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XChar2b {
    pub byte1: ::std::os::raw::c_uchar,
    pub byte2: ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XChar2b"][::std::mem::size_of::<XChar2b>() - 2usize];
    ["Alignment of XChar2b"][::std::mem::align_of::<XChar2b>() - 1usize];
    ["Offset of field: XChar2b::byte1"][::std::mem::offset_of!(XChar2b, byte1) - 0usize];
    ["Offset of field: XChar2b::byte2"][::std::mem::offset_of!(XChar2b, byte2) - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XTextItem16 {
    pub chars: *mut XChar2b,
    pub nchars: ::std::os::raw::c_int,
    pub delta: ::std::os::raw::c_int,
    pub font: Font,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XTextItem16"][::std::mem::size_of::<XTextItem16>() - 24usize];
    ["Alignment of XTextItem16"][::std::mem::align_of::<XTextItem16>() - 8usize];
    ["Offset of field: XTextItem16::chars"][::std::mem::offset_of!(XTextItem16, chars) - 0usize];
    ["Offset of field: XTextItem16::nchars"][::std::mem::offset_of!(XTextItem16, nchars) - 8usize];
    ["Offset of field: XTextItem16::delta"][::std::mem::offset_of!(XTextItem16, delta) - 12usize];
    ["Offset of field: XTextItem16::font"][::std::mem::offset_of!(XTextItem16, font) - 16usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union XEDataObject {
    pub display: *mut Display,
    pub gc: GC,
    pub visual: *mut Visual,
    pub screen: *mut Screen,
    pub pixmap_format: *mut ScreenFormat,
    pub font: *mut XFontStruct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XEDataObject"][::std::mem::size_of::<XEDataObject>() - 8usize];
    ["Alignment of XEDataObject"][::std::mem::align_of::<XEDataObject>() - 8usize];
    ["Offset of field: XEDataObject::display"]
        [::std::mem::offset_of!(XEDataObject, display) - 0usize];
    ["Offset of field: XEDataObject::gc"][::std::mem::offset_of!(XEDataObject, gc) - 0usize];
    ["Offset of field: XEDataObject::visual"]
        [::std::mem::offset_of!(XEDataObject, visual) - 0usize];
    ["Offset of field: XEDataObject::screen"]
        [::std::mem::offset_of!(XEDataObject, screen) - 0usize];
    ["Offset of field: XEDataObject::pixmap_format"]
        [::std::mem::offset_of!(XEDataObject, pixmap_format) - 0usize];
    ["Offset of field: XEDataObject::font"][::std::mem::offset_of!(XEDataObject, font) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XFontSetExtents {
    pub max_ink_extent: XRectangle,
    pub max_logical_extent: XRectangle,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XFontSetExtents"][::std::mem::size_of::<XFontSetExtents>() - 16usize];
    ["Alignment of XFontSetExtents"][::std::mem::align_of::<XFontSetExtents>() - 2usize];
    ["Offset of field: XFontSetExtents::max_ink_extent"]
        [::std::mem::offset_of!(XFontSetExtents, max_ink_extent) - 0usize];
    ["Offset of field: XFontSetExtents::max_logical_extent"]
        [::std::mem::offset_of!(XFontSetExtents, max_logical_extent) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XOM {
    _unused: [u8; 0],
}
pub type XOM = *mut _XOM;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XOC {
    _unused: [u8; 0],
}
pub type XOC = *mut _XOC;
pub type XFontSet = *mut _XOC;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XmbTextItem {
    pub chars: *mut ::std::os::raw::c_char,
    pub nchars: ::std::os::raw::c_int,
    pub delta: ::std::os::raw::c_int,
    pub font_set: XFontSet,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XmbTextItem"][::std::mem::size_of::<XmbTextItem>() - 24usize];
    ["Alignment of XmbTextItem"][::std::mem::align_of::<XmbTextItem>() - 8usize];
    ["Offset of field: XmbTextItem::chars"][::std::mem::offset_of!(XmbTextItem, chars) - 0usize];
    ["Offset of field: XmbTextItem::nchars"][::std::mem::offset_of!(XmbTextItem, nchars) - 8usize];
    ["Offset of field: XmbTextItem::delta"][::std::mem::offset_of!(XmbTextItem, delta) - 12usize];
    ["Offset of field: XmbTextItem::font_set"]
        [::std::mem::offset_of!(XmbTextItem, font_set) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XwcTextItem {
    pub chars: *mut wchar_t,
    pub nchars: ::std::os::raw::c_int,
    pub delta: ::std::os::raw::c_int,
    pub font_set: XFontSet,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XwcTextItem"][::std::mem::size_of::<XwcTextItem>() - 24usize];
    ["Alignment of XwcTextItem"][::std::mem::align_of::<XwcTextItem>() - 8usize];
    ["Offset of field: XwcTextItem::chars"][::std::mem::offset_of!(XwcTextItem, chars) - 0usize];
    ["Offset of field: XwcTextItem::nchars"][::std::mem::offset_of!(XwcTextItem, nchars) - 8usize];
    ["Offset of field: XwcTextItem::delta"][::std::mem::offset_of!(XwcTextItem, delta) - 12usize];
    ["Offset of field: XwcTextItem::font_set"]
        [::std::mem::offset_of!(XwcTextItem, font_set) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XOMCharSetList {
    pub charset_count: ::std::os::raw::c_int,
    pub charset_list: *mut *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XOMCharSetList"][::std::mem::size_of::<XOMCharSetList>() - 16usize];
    ["Alignment of XOMCharSetList"][::std::mem::align_of::<XOMCharSetList>() - 8usize];
    ["Offset of field: XOMCharSetList::charset_count"]
        [::std::mem::offset_of!(XOMCharSetList, charset_count) - 0usize];
    ["Offset of field: XOMCharSetList::charset_list"]
        [::std::mem::offset_of!(XOMCharSetList, charset_list) - 8usize];
};
pub const XOrientation_XOMOrientation_LTR_TTB: XOrientation = 0;
pub const XOrientation_XOMOrientation_RTL_TTB: XOrientation = 1;
pub const XOrientation_XOMOrientation_TTB_LTR: XOrientation = 2;
pub const XOrientation_XOMOrientation_TTB_RTL: XOrientation = 3;
pub const XOrientation_XOMOrientation_Context: XOrientation = 4;
pub type XOrientation = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XOMOrientation {
    pub num_orientation: ::std::os::raw::c_int,
    pub orientation: *mut XOrientation,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XOMOrientation"][::std::mem::size_of::<XOMOrientation>() - 16usize];
    ["Alignment of XOMOrientation"][::std::mem::align_of::<XOMOrientation>() - 8usize];
    ["Offset of field: XOMOrientation::num_orientation"]
        [::std::mem::offset_of!(XOMOrientation, num_orientation) - 0usize];
    ["Offset of field: XOMOrientation::orientation"]
        [::std::mem::offset_of!(XOMOrientation, orientation) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XOMFontInfo {
    pub num_font: ::std::os::raw::c_int,
    pub font_struct_list: *mut *mut XFontStruct,
    pub font_name_list: *mut *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XOMFontInfo"][::std::mem::size_of::<XOMFontInfo>() - 24usize];
    ["Alignment of XOMFontInfo"][::std::mem::align_of::<XOMFontInfo>() - 8usize];
    ["Offset of field: XOMFontInfo::num_font"]
        [::std::mem::offset_of!(XOMFontInfo, num_font) - 0usize];
    ["Offset of field: XOMFontInfo::font_struct_list"]
        [::std::mem::offset_of!(XOMFontInfo, font_struct_list) - 8usize];
    ["Offset of field: XOMFontInfo::font_name_list"]
        [::std::mem::offset_of!(XOMFontInfo, font_name_list) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIM {
    _unused: [u8; 0],
}
pub type XIM = *mut _XIM;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIC {
    _unused: [u8; 0],
}
pub type XIC = *mut _XIC;
pub type XIMProc =
    ::std::option::Option<unsafe extern "C" fn(arg1: XIM, arg2: XPointer, arg3: XPointer)>;
pub type XICProc = ::std::option::Option<
    unsafe extern "C" fn(arg1: XIC, arg2: XPointer, arg3: XPointer) -> ::std::os::raw::c_int,
>;
pub type XIDProc =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut Display, arg2: XPointer, arg3: XPointer)>;
pub type XIMStyle = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMStyles {
    pub count_styles: ::std::os::raw::c_ushort,
    pub supported_styles: *mut XIMStyle,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XIMStyles"][::std::mem::size_of::<XIMStyles>() - 16usize];
    ["Alignment of XIMStyles"][::std::mem::align_of::<XIMStyles>() - 8usize];
    ["Offset of field: XIMStyles::count_styles"]
        [::std::mem::offset_of!(XIMStyles, count_styles) - 0usize];
    ["Offset of field: XIMStyles::supported_styles"]
        [::std::mem::offset_of!(XIMStyles, supported_styles) - 8usize];
};
pub type XVaNestedList = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XIMCallback {
    pub client_data: XPointer,
    pub callback: XIMProc,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XIMCallback"][::std::mem::size_of::<XIMCallback>() - 16usize];
    ["Alignment of XIMCallback"][::std::mem::align_of::<XIMCallback>() - 8usize];
    ["Offset of field: XIMCallback::client_data"]
        [::std::mem::offset_of!(XIMCallback, client_data) - 0usize];
    ["Offset of field: XIMCallback::callback"]
        [::std::mem::offset_of!(XIMCallback, callback) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XICCallback {
    pub client_data: XPointer,
    pub callback: XICProc,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of XICCallback"][::std::mem::size_of::<XICCallback>() - 16usize];
    ["Alignment of XICCallback"][::std::mem::align_of::<XICCallback>() - 8usize];
    ["Offset of field: XICCallback::client_data"]
        [::std::mem::offset_of!(XICCallback, client_data) - 0usize];
    ["Offset of field: XICCallback::callback"]
        [::std::mem::offset_of!(XICCallback, callback) - 8usize];
};
pub type XIMFeedback = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XIMText {
    pub length: ::std::os::raw::c_ushort,
    pub feedback: *mut XIMFeedback,
    pub encoding_is_wchar: ::std::os::raw::c_int,
    pub string: _XIMText__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XIMText__bindgen_ty_1 {
    pub multi_byte: *mut ::std::os::raw::c_char,
    pub wide_char: *mut wchar_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XIMText__bindgen_ty_1"][::std::mem::size_of::<_XIMText__bindgen_ty_1>() - 8usize];
    ["Alignment of _XIMText__bindgen_ty_1"]
        [::std::mem::align_of::<_XIMText__bindgen_ty_1>() - 8usize];
    ["Offset of field: _XIMText__bindgen_ty_1::multi_byte"]
        [::std::mem::offset_of!(_XIMText__bindgen_ty_1, multi_byte) - 0usize];
    ["Offset of field: _XIMText__bindgen_ty_1::wide_char"]
        [::std::mem::offset_of!(_XIMText__bindgen_ty_1, wide_char) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XIMText"][::std::mem::size_of::<_XIMText>() - 32usize];
    ["Alignment of _XIMText"][::std::mem::align_of::<_XIMText>() - 8usize];
    ["Offset of field: _XIMText::length"][::std::mem::offset_of!(_XIMText, length) - 0usize];
    ["Offset of field: _XIMText::feedback"][::std::mem::offset_of!(_XIMText, feedback) - 8usize];
    ["Offset of field: _XIMText::encoding_is_wchar"]
        [::std::mem::offset_of!(_XIMText, encoding_is_wchar) - 16usize];
    ["Offset of field: _XIMText::string"][::std::mem::offset_of!(_XIMText, string) - 24usize];
};
pub type XIMText = _XIMText;
pub type XIMPreeditState = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIMPreeditStateNotifyCallbackStruct {
    pub state: XIMPreeditState,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XIMPreeditStateNotifyCallbackStruct"]
        [::std::mem::size_of::<_XIMPreeditStateNotifyCallbackStruct>() - 8usize];
    ["Alignment of _XIMPreeditStateNotifyCallbackStruct"]
        [::std::mem::align_of::<_XIMPreeditStateNotifyCallbackStruct>() - 8usize];
    ["Offset of field: _XIMPreeditStateNotifyCallbackStruct::state"]
        [::std::mem::offset_of!(_XIMPreeditStateNotifyCallbackStruct, state) - 0usize];
};
pub type XIMPreeditStateNotifyCallbackStruct = _XIMPreeditStateNotifyCallbackStruct;
pub type XIMResetState = ::std::os::raw::c_ulong;
pub type XIMStringConversionFeedback = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XIMStringConversionText {
    pub length: ::std::os::raw::c_ushort,
    pub feedback: *mut XIMStringConversionFeedback,
    pub encoding_is_wchar: ::std::os::raw::c_int,
    pub string: _XIMStringConversionText__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XIMStringConversionText__bindgen_ty_1 {
    pub mbs: *mut ::std::os::raw::c_char,
    pub wcs: *mut wchar_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XIMStringConversionText__bindgen_ty_1"]
        [::std::mem::size_of::<_XIMStringConversionText__bindgen_ty_1>() - 8usize];
    ["Alignment of _XIMStringConversionText__bindgen_ty_1"]
        [::std::mem::align_of::<_XIMStringConversionText__bindgen_ty_1>() - 8usize];
    ["Offset of field: _XIMStringConversionText__bindgen_ty_1::mbs"]
        [::std::mem::offset_of!(_XIMStringConversionText__bindgen_ty_1, mbs) - 0usize];
    ["Offset of field: _XIMStringConversionText__bindgen_ty_1::wcs"]
        [::std::mem::offset_of!(_XIMStringConversionText__bindgen_ty_1, wcs) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XIMStringConversionText"]
        [::std::mem::size_of::<_XIMStringConversionText>() - 32usize];
    ["Alignment of _XIMStringConversionText"]
        [::std::mem::align_of::<_XIMStringConversionText>() - 8usize];
    ["Offset of field: _XIMStringConversionText::length"]
        [::std::mem::offset_of!(_XIMStringConversionText, length) - 0usize];
    ["Offset of field: _XIMStringConversionText::feedback"]
        [::std::mem::offset_of!(_XIMStringConversionText, feedback) - 8usize];
    ["Offset of field: _XIMStringConversionText::encoding_is_wchar"]
        [::std::mem::offset_of!(_XIMStringConversionText, encoding_is_wchar) - 16usize];
    ["Offset of field: _XIMStringConversionText::string"]
        [::std::mem::offset_of!(_XIMStringConversionText, string) - 24usize];
};
pub type XIMStringConversionText = _XIMStringConversionText;
pub type XIMStringConversionPosition = ::std::os::raw::c_ushort;
pub type XIMStringConversionType = ::std::os::raw::c_ushort;
pub type XIMStringConversionOperation = ::std::os::raw::c_ushort;
pub const XIMCaretDirection_XIMForwardChar: XIMCaretDirection = 0;
pub const XIMCaretDirection_XIMBackwardChar: XIMCaretDirection = 1;
pub const XIMCaretDirection_XIMForwardWord: XIMCaretDirection = 2;
pub const XIMCaretDirection_XIMBackwardWord: XIMCaretDirection = 3;
pub const XIMCaretDirection_XIMCaretUp: XIMCaretDirection = 4;
pub const XIMCaretDirection_XIMCaretDown: XIMCaretDirection = 5;
pub const XIMCaretDirection_XIMNextLine: XIMCaretDirection = 6;
pub const XIMCaretDirection_XIMPreviousLine: XIMCaretDirection = 7;
pub const XIMCaretDirection_XIMLineStart: XIMCaretDirection = 8;
pub const XIMCaretDirection_XIMLineEnd: XIMCaretDirection = 9;
pub const XIMCaretDirection_XIMAbsolutePosition: XIMCaretDirection = 10;
pub const XIMCaretDirection_XIMDontChange: XIMCaretDirection = 11;
pub type XIMCaretDirection = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIMStringConversionCallbackStruct {
    pub position: XIMStringConversionPosition,
    pub direction: XIMCaretDirection,
    pub operation: XIMStringConversionOperation,
    pub factor: ::std::os::raw::c_ushort,
    pub text: *mut XIMStringConversionText,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XIMStringConversionCallbackStruct"]
        [::std::mem::size_of::<_XIMStringConversionCallbackStruct>() - 24usize];
    ["Alignment of _XIMStringConversionCallbackStruct"]
        [::std::mem::align_of::<_XIMStringConversionCallbackStruct>() - 8usize];
    ["Offset of field: _XIMStringConversionCallbackStruct::position"]
        [::std::mem::offset_of!(_XIMStringConversionCallbackStruct, position) - 0usize];
    ["Offset of field: _XIMStringConversionCallbackStruct::direction"]
        [::std::mem::offset_of!(_XIMStringConversionCallbackStruct, direction) - 4usize];
    ["Offset of field: _XIMStringConversionCallbackStruct::operation"]
        [::std::mem::offset_of!(_XIMStringConversionCallbackStruct, operation) - 8usize];
    ["Offset of field: _XIMStringConversionCallbackStruct::factor"]
        [::std::mem::offset_of!(_XIMStringConversionCallbackStruct, factor) - 10usize];
    ["Offset of field: _XIMStringConversionCallbackStruct::text"]
        [::std::mem::offset_of!(_XIMStringConversionCallbackStruct, text) - 16usize];
};
pub type XIMStringConversionCallbackStruct = _XIMStringConversionCallbackStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIMPreeditDrawCallbackStruct {
    pub caret: ::std::os::raw::c_int,
    pub chg_first: ::std::os::raw::c_int,
    pub chg_length: ::std::os::raw::c_int,
    pub text: *mut XIMText,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _XIMPreeditDrawCallbackStruct"]
        [::std::mem::size_of::<_XIMPreeditDrawCallbackStruct>() - 24usize];
    ["Alignment of _XIMPreeditDrawCallbackStruct"]
        [::std::mem::align_of::<_XIMPreeditDrawCallbackStruct>() - 8usize];
    ["Offset of field: _XIMPreeditDrawCallbackStruct::caret"]
        [::std::mem::offset_of!(_XIMPreeditDrawCallbackStruct, caret) - 0usize];
    ["Offset of field: _XIMPreeditDrawCallbackStruct::chg_first"]
        [::std::mem::offset_of!(_XIMPreeditDrawCallbackStruct, chg_first) - 4usize];
    ["Offset of field: _XIMPreeditDrawCallbackStruct::chg_length"]
        [::std::mem::offset_of!(_XIMPreeditDrawCallbackStruct, chg_length) - 8usize];
    ["Offset of field: _XIMPreeditDrawCallbackStruct::text"]
        [::std::mem::offset_of!(_XIMPreeditDrawCallbackStruct, text) - 16usize];
};
pub type XIMPreeditDrawCallbackStruct = _XIMPreeditDrawCallbackStruct;
pub const XIMCaretStyle_XIMIsInvisible: XIMCaretStyle = 0;
pub const XIMCaretStyle_XIMIsPrimary: XIMCaretStyle = 1;
pub const XIMCaretStyle_XIMIsSecondary: XIMCaretStyle = 2;
pub type XIMCaretStyle = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XIMPreeditCaretCallbackStruct {
    pub position: ::std::os::raw::c_int,
    pub direction: XIMCaretDirection,
    pub style: XIMCaretStyle,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
// const _: () = {
//     ["Size of _XIMPreeditCaretCallbackStruct"]
//         [::std::mem::size_of::<_XIMPreeditCaretCallbackStruct>() - 12usize];
//     ["Alignment of _XIMPreeditCaretCallbackStruct"]
//         [::std::mem::align_of::<_XIMPreeditCaretCallbackStruct>() - 4usize];
//     ["Offset of field: _XIMPreeditCaretCallbackStruct::position"]
//         [::std::mem::offset_of!(_XIMPreeditCaretCallbackStruct, position) - 0usize];
//     ["Offset of field: _XIMPreeditCaretCallbackStruct::direction"]
//         [::std::mem::offset_of!(_XIMPreeditCaretCallbackStruct, direction) - 4usize];
//     ["Offset of field: _XIMPreeditCaretCallbackStruct::style"]
//         [::std::mem::offset_of!(_XIMPreeditCaretCallbackStruct, style) - 8usize];
// };
pub type XIMPreeditCaretCallbackStruct = _XIMPreeditCaretCallbackStruct;
pub const XIMStatusDataType_XIMTextType: XIMStatusDataType = 0;
pub const XIMStatusDataType_XIMBitmapType: XIMStatusDataType = 1;
pub type XIMStatusDataType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XIMStatusDrawCallbackStruct {
    pub type_: XIMStatusDataType,
    pub data: _XIMStatusDrawCallbackStruct__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _XIMStatusDrawCallbackStruct__bindgen_ty_1 {
    pub text: *mut XIMText,
    pub bitmap: Pixmap,
}
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