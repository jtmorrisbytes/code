impl RenderTarget {
    pub fn windowed<Window>(w: &Window) -> Self
    where
        Window: HasDisplayHandle + HasWindowHandle,
    {
        let display_handle = w.display_handle().unwrap().as_raw();
        let window_handle = w.window_handle().unwrap().as_raw();
        Self::Windowed(window_handle, display_handle)
    }
    pub fn headless() -> Self {
        Self::Headless
    }
}

pub enum RenderTarget {
    Windowed(RawWindowHandle, RawDisplayHandle),
    Headless,
}

pub struct BackendOptions {
    pub render_target: RenderTarget,
}




// graphicsbackend should be a trait used to intitialize and interact with any graphics api
// while allowing common interactions
pub trait GraphicsBackend {
    // // creates an instance of this structure. after this call, the backend strucutre must be ready to accept a call to initialize
    // fn new() -> Result<Self,anyhow::Error> where Self:Sized;
    /// creates an instance of this backend. after the call to this function, the backend should be ready to accept drawing commands
    fn init(render_target:RenderTarget) -> anyhow::Result<Self> where Self:Sized;
    fn resize(&mut self, width: u32, height: u32);
    fn render_frame(&mut self, scene: ());
    fn shutdown(&mut self);
    /// called on mobile platforms whenever the app is suspended and the 'surface' may need to be destroyed
    fn suspend(&mut self);
    /// called on mobile platforms whenever the app is resumed from the background and the 'surface' may need to be recreated
    fn resume(&mut self, options: &BackendOptions);
}
use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WindowHandle,
};
