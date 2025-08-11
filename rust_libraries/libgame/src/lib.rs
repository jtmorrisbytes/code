use raw_window_handle::{HasDisplayHandle, HasRawWindowHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};


// I want to write my own graphics backends in Vulkan (primary) and opengl

#[cfg(target_os = "windows")]
pub mod platform_windows;

pub mod engine;
pub mod backends;