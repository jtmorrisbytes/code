pub struct Application {
    window: Option<winit::window::Window>,
    cursor_x:f64,
    cursor_y:f64,
    width:u32,
    height:u32,
    has_focus:bool
}

impl Application {
    pub fn new() -> Self {
        Application { window:None,cursor_x:0.0,cursor_y:0.0,width:1,height:1,has_focus:false }
    }
    pub fn draw(&self) {

    }
}



cfg_if::cfg_if! {
    if #[cfg(feature="target_os_linux")] {
        pub mod linux;
    }
    else if  #[cfg(feature="target_os_windows")] {
        pub mod windows;
    }
}

pub mod gfx;


#[derive(thiserror::Error,Debug)]
#[error("CreateEventLoopError:{0}")]
pub enum CreateEventLoopError {
    #[error("Cannot determine what kind of desktop session you are running. {0}")]
    LinuxCannotDetermineSessionType(String),
    #[error("Unsupported desktop session type: {0}. Supported types are 'x11' and 'wayland'")]
    LinuxUnsupportedSessionType(String),
    #[error("Event loop error: {0}")]
    EventLoopError(String)
}

pub fn create_event_loop() -> Result<winit::event_loop::EventLoop<()>,CreateEventLoopError>  {
    
        let mut event_loop_builder = winit::event_loop::EventLoop::builder();
        let mut event_loop_builder_ref = &mut event_loop_builder;
        #[cfg(feature="target_os_linux")]{
        // attempt to determine the type of desktop session we are running. supports 'x11' or 'wayland'

        use winit::platform::{wayland::EventLoopBuilderExtWayland, x11::EventLoopBuilderExtX11};
        let xdg_session_type =std::env::var("XDG_SESSION_TYPE").inspect_err(|e| {
            eprintln!("Error while querying XDG_SESION_TYPE: {e}");
        }).map_err(|e| CreateEventLoopError::LinuxCannotDetermineSessionType(e.to_string()))?;
        match xdg_session_type.as_str() {
            "x11" => event_loop_builder_ref = event_loop_builder_ref.with_x11(),
            "wayland" =>event_loop_builder_ref = event_loop_builder_ref.with_wayland(),
            _=>{return Err(CreateEventLoopError::LinuxUnsupportedSessionType(xdg_session_type))}
        };
        }
        #[cfg(feature="target_os_windows")] {
            // use winit::platform::windows::{EventLoopBuilderExtWindows};
            // windows specific code here
        }
        event_loop_builder_ref.build().map_err(|e| CreateEventLoopError::EventLoopError(e.to_string()))
}