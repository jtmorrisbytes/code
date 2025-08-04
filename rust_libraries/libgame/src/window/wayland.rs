use std::os::unix::net::UnixStream;
use std::io::{Read,Write};
// compile_error!("finish wayland client")
// i know this is probably really not smart, but I want my own rust-only wayland client
//

fn get_env_wayland_display() -> Option<String> {
    match std::env::var("WAYLAND_DISPLAY") {
        Err(std::env::VarError::NotPresent) => {
            return None;
        }
        Err(std::env::VarError::NotUnicode(os_string)) => {
            Some(os_string.to_string_lossy().to_string())
        }
        Ok(s) => Some(s),
    }
}
fn get_env_xdg_runtime_dir() -> Option<String> {
    match std::env::var("XDG_RUNTIME_DIR") {
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(os_string)) => {
            Some(os_string.to_string_lossy().to_string())
        }
        Ok(s) => Some(s),
    }
}

fn fmt_socket_path(xdg_runtime_dir: &str, wayland_display: &str) -> String {
    format!("{xdg_runtime_dir}/{wayland_display}")
}
// #[inline(always)]
// fn log_error(s: &str) {
//     #[cfg(debug_assertions)]
//     eprintln!("Error: {s}");
// }

#[derive(Debug)]
pub enum ConnectError {
    XdgRuntimeDirEnvNotPresent,
    SocketIoError(std::io::Error),
}

/// attempts to create a unix socket at $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY or $XDG_RUNTIME_DIR/wayland-0
/// and returns an error if it fails
pub fn connect() -> Result<UnixStream, ConnectError> {
    let xdg_runtime_dir = match get_env_xdg_runtime_dir() {
        Some(s) => s,
        None => {
            return Err(ConnectError::XdgRuntimeDirEnvNotPresent);
        }
    };

    let wayland_display = match get_env_wayland_display() {
        Some(s) => s,
        None => "wayland-0".to_string(),
    };
    let socket_path = fmt_socket_path(&xdg_runtime_dir, &wayland_display);
    UnixStream::connect(socket_path).map_err(|e| ConnectError::SocketIoError(e))
}

pub type ObjectId = u32;
pub type Opcode = u16;
pub type MessageSize = u16;
pub type Buffer = Vec<u8>;

// fn buf_write_object_id(buffer: &mut Buffer, object_id:ObjectId) {
//     let mut oid_bytes = object_id.to_ne_bytes();
//     buffer.extend_from_slice(&mut oid_bytes);
// }
// fn buf_write_message_size(buffer: &mut Buffer,message_size:MessageSize) {
//     let mut message_size_bytes = message_size.to_ne_bytes();
//     buffer.extend_from_slice(&mut message_size_bytes);
// }


fn header(object_id: ObjectId,message_size:MessageSize,opcode:Opcode) -> [u8;64] {
    let mut buf: [u8;64] = [0_u8;64];
    
    let oid_bytes = object_id.to_ne_bytes();
    buf[0] = oid_bytes[0];
    buf[1] = oid_bytes[1];
    buf[2] = oid_bytes[2];
    buf[3] = oid_bytes[3];

    let message_size_bytes = message_size.to_ne_bytes();
    buf[4] = message_size_bytes[0];
    buf[5] = message_size_bytes[1];

    let opcode_bytes = opcode.to_ne_bytes();
    buf[6] = opcode_bytes[0];
    buf[7] = opcode_bytes[1];
    return buf;   
}



pub const WAYLAND_DISPLAY_OBJECT_ID: ObjectId = 1;
pub const WAYLAND_WL_DISPLAY_GET_REGISTRY_OPCODE: Opcode = 1;


pub struct Client {
    current_wayland_id: ObjectId,
    socket: UnixStream
}
impl Client {
    /// attempts to create a unix socket at $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY or $XDG_RUNTIME_DIR/wayland-0
    /// and returns an error if it fails
    pub fn connect() -> Result<Self, ConnectError> {
        let xdg_runtime_dir = match get_env_xdg_runtime_dir() {
            Some(s) => s,
            None => {
                return Err(ConnectError::XdgRuntimeDirEnvNotPresent);
            }
        };

        let wayland_display = match get_env_wayland_display() {
            Some(s) => s,
            None => "wayland-0".to_string(),
        };
        let socket_path = fmt_socket_path(&xdg_runtime_dir, &wayland_display);
        Ok(Self {
            socket: UnixStream::connect(socket_path).map_err(|e| ConnectError::SocketIoError(e))?,
            current_wayland_id: 0,
        })
    }
    pub fn wayland_wl_display_get_registry(&mut self) {
        // let _ = &self.socket;
        let _ = &self.current_wayland_id;

        // create and format the message
        let mut message: Vec<u8> = vec![];        
        message.extend_from_slice(&header(WAYLAND_DISPLAY_OBJECT_ID,64,WAYLAND_WL_DISPLAY_GET_REGISTRY_OPCODE));

        self.current_wayland_id = self.current_wayland_id + 1;
        let _ = self.socket.write_all(&message).unwrap();

        let mut response = vec![0_u8;64];
        let _ = self.socket.read(&mut response);
        
        println!("{response:?}");
    }
}

#[test]
pub fn wayland_test_bindings() {
    let mut client = self::Client::connect().unwrap();
    let _ = client.wayland_wl_display_get_registry();

}
