#![allow(non_camel_case_types)]

use crate::bindings::{
    VkAllocationCallbacks, VkBool32, VkDevice, VkDeviceGroupPresentModeFlagsKHR, VkDeviceMemory,
    VkDisplayKHR, VkExternalFenceHandleTypeFlagBits, VkExternalMemoryHandleTypeFlagBits,
    VkExternalMemoryHandleTypeFlagsNV, VkExternalSemaphoreHandleTypeFlagBits, VkFence,
    VkFenceImportFlags, VkInstance, VkPhysicalDevice, VkPhysicalDeviceSurfaceInfo2KHR,
    VkPresentModeKHR, VkResult, VkSemaphore, VkSemaphoreImportFlags, VkStructureType, VkSurfaceKHR,
    VkSwapchainKHR,
};

type VkWin32SurfaceCreateFlagsKHR = crate::bindings::VkFlags;

// using the official windows-rs for type defs bc i dont want to have to define them myself
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{HANDLE, HINSTANCE, HMODULE, HWND};
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Security::SECURITY_ATTRIBUTES;

// type DWORD is not available from the windows crate
type DWORD = u32;

// filler types for now, maybe depend on windows-rs? or get them from windows.h

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    flags: VkWin32SurfaceCreateFlagsKHR,
    hinstance: HINSTANCE,
    hwnd: HWND,
}

pub type PFN_vkCreateWin32SurfaceKHR = Option<
    unsafe extern "C" fn(
        instance: VkInstance,
        pCreateInfo: *mut VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult,
>;

pub const VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &std::ffi::CStr =
    c"VK_KHR_external_memory_win32";
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: std::ffi::c_int = 1;

pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: std::ffi::c_int = 6;
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &std::ffi::CStr = c"VK_KHR_win32_surface";

// type PFN_vkCreateWin32SurfaceKHR = Option<unsafe extern "C" fn(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult>;
type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = Option<
    unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    name: PCWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    pAttributes: *const SECURITY_ATTRIBUTES,
    dwAccess: DWORD,
    name: PCWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    sType: VkStructureType,
    pNext: *mut std::ffi::c_void,
    memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagBits,
}

type PFN_vkGetMemoryWin32HandleKHR = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;
type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        handleType: VkExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
    ) -> VkResult,
>;

const VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: std::ffi::c_int = 1;
const VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &std::ffi::CStr = c"VK_KHR_win32_keyed_mutex";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    acquireCount: u32,
    pAcquireSyncs: *const VkDeviceMemory,
    pAcquireKeys: *const u64,
    pAcquireTimeouts: *const u32,
    releaseCount: u32,
    pReleaseSyncs: *const VkDeviceMemory,
    pReleaseKeys: *const u64,
}

const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: std::ffi::c_int = 1;
const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &std::ffi::CStr =
    c"VK_KHR_external_semaphore_win32";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    semaphore: VkSemaphore,
    flags: VkSemaphoreImportFlags,
    handleType: VkExternalSemaphoreHandleTypeFlagBits,
    handle: HANDLE,
    name: PCWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    pAttributes: *const SECURITY_ATTRIBUTES,
    dwAccess: DWORD,
    name: PCWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkD3D12FenceSubmitInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    waitSemaphoreValuesCount: u32,
    pWaitSemaphoreValues: *const u64,
    signalSemaphoreValuesCount: u32,
    pSignalSemaphoreValues: *const u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    semaphore: VkSemaphore,
    handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

type PFN_vkImportSemaphoreWin32HandleKHR = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
    ) -> VkResult,
>;
type PFN_vkGetSemaphoreWin32HandleKHR = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;

const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: std::ffi::c_int = 1;
const VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &std::ffi::CStr = c"VK_KHR_external_fence_win32";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportFenceWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    fence: VkFence,
    flags: VkFenceImportFlags,
    handleType: VkExternalFenceHandleTypeFlagBits,
    handle: HANDLE,
    name: PCWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportFenceWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    pAttributes: *const SECURITY_ATTRIBUTES,
    dwAccess: DWORD,
    name: PCWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFenceGetWin32HandleInfoKHR {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    fence: VkFence,
    handleType: VkExternalFenceHandleTypeFlagBits,
}

type PFN_vkImportFenceWin32HandleKHR = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
    ) -> VkResult,
>;
type PFN_vkGetFenceWin32HandleKHR = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;

const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: std::ffi::c_int = 1;
const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &std::ffi::CStr = c"VK_NV_external_memory_win32";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImportMemoryWin32HandleInfoNV {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    handle: HANDLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExportMemoryWin32HandleInfoNV {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    pAttributes: *const SECURITY_ATTRIBUTES,
    dwAccess: DWORD,
}

type PFN_vkGetMemoryWin32HandleNV = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        memory: VkDeviceMemory,
        handleType: VkExternalMemoryHandleTypeFlagsNV,
        pHandle: *mut HANDLE,
    ) -> VkResult,
>;

const VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION: std::ffi::c_int = 2;
const VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: &std::ffi::CStr = c"VK_NV_win32_keyed_mutex";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    acquireCount: u32,
    pAcquireSyncs: *const VkDeviceMemory,
    pAcquireKeys: *const u64,
    pAcquireTimeoutMilliseconds: *const u32,
    releaseCount: u32,
    pReleaseSyncs: *const VkDeviceMemory,
    pReleaseKeys: *const u64,
}

const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: std::ffi::c_int = 4;
const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &std::ffi::CStr =
    c"VK_EXT_full_screen_exclusive";

#[repr(u32)]
#[derive(Clone, PartialEq, Eq, Copy)]
pub enum VkFullScreenExclusiveEXT {
    VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0,
    VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1,
    VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2,
    VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3,
    VK_FULL_SCREEN_EXCLUSIVE_MAX_ENUM_EXT = 0x7FFFFFFF,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    sType: VkStructureType,
    pNext: *mut std::ffi::c_void,
    fullScreenExclusive: VkFullScreenExclusiveEXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    sType: VkStructureType,
    pNext: *mut std::ffi::c_void,
    fullScreenExclusiveSupported: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
    sType: VkStructureType,
    pNext: *const std::ffi::c_void,
    hmonitor: HMONITOR,
}

type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = Option<
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResult,
>;
type PFN_vkAcquireFullScreenExclusiveModeEXT =
    Option<unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult>;
type PFN_vkReleaseFullScreenExclusiveModeEXT =
    Option<unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult>;
type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
        pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
    ) -> VkResult,
>;

const VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: std::ffi::c_int = 1;
const VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &std::ffi::CStr = c"VK_NV_acquire_winrt_display";
type PFN_vkAcquireWinrtDisplayNV = Option<
    unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult,
>;
type PFN_vkGetWinrtDisplayNV = Option<
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        deviceRelativeId: u32,
        pDisplay: *mut VkDisplayKHR,
    ) -> VkResult,
>;

// a wrapper for loadlibraryA
pub unsafe fn _dlopen(path: &str) -> Result<*mut std::ffi::c_void, String> {
    // convert the rust str to a CSTRING
    let cstring = std::ffi::CString::new(path)
        .map_err(|e| format!("Recieved null terminated rust &str. DO NOT DO THIS. error: {e}"))?;
    // convert into a byte slice &[u8]
    let bytes = cstring.as_bytes_with_nul();
    // create a pointer to *const u8
    let pcstr = PCSTR::from_raw(bytes.as_ptr());
    /// use loadlibraryA to aquire the handle to the module
    let hmodule =
        windows::Win32::System::LibraryLoader::LoadLibraryA(pcstr).map_err(|e| e.to_string())?;
    // the underlying type of a HMODULE is a *mut std::ffi::c_void pointer at field 0
    let ptr = hmodule.0;

    Ok(ptr)
}

pub unsafe fn _dlsym(
    handle: *mut std::ffi::c_void,
    symbol_name: &str,
) -> Result<*mut std::ffi::c_void, String> {
    let module = HMODULE(handle);
    if module.is_invalid() {
        return Err(format!("The pointer at {handle:?} is invalid."));
    }

    todo!()
}

pub unsafe fn _dlerror() -> Option<String> {
    // we return none here because we have handled any errors at the win32 level
    None
}
pub unsafe fn _dlclose(handle: *mut std::ffi::c_void) {
    todo!()
}
