
pub mod backends;
pub mod engine;

// I want to write my own graphics backends in Vulkan (primary) and opengl

#[cfg(target_os = "windows")]
pub mod platform_windows;
use raw_window_handle::{HasDisplayHandle, HasRawWindowHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};
