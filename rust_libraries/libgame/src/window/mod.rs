// create a window or 'surface' to be able to call some drawing apis
// targeting support for windows: win32
// linux: x11 + wayland
#[cfg(unix)]
pub mod wayland;