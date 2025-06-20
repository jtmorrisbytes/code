#![deny(unused_import_braces, unused_imports, warnings)]

pub mod frontend;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod web_server;
