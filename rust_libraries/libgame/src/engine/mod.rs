use crate::{
    backends::vulkan::VulkanBackend,
    engine::backend::{BackendOptions, GraphicsBackend, RenderTarget},
};

pub mod backend;

pub enum Backend {
    Vulkan(super::backends::vulkan::VulkanBackend),
    OpenGL(()),
    Headless(()),
}
impl backend::GraphicsBackend for Backend {
    fn init(&mut self, options: &backend::BackendOptions) -> anyhow::Result<()> {
        match self {
            // initialize vulkan. if the render target is the window, then we need to create the surface from here
            Backend::Vulkan(b) => {
                b
                .initialize()
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;
                match options.render_target {
                    RenderTarget::Windowed(window_handle,display_handle) => {
                        b.instance_create_surface(window_handle,display_handle).map_err(|e| anyhow::Error::msg(e.to_string()))
                    },
                    RenderTarget::Headless=>{Ok(())}
                }
            
            },
                
            Backend::OpenGL(_) => Ok(()),
            Backend::Headless(_) => Ok(()),
        }
    }
    fn render_frame(&mut self, scene: ()) {}
    fn resize(&mut self, width: u32, height: u32) {}

    fn shutdown(&mut self) {}

    fn suspend(&mut self) {
        match self {
            Backend::Vulkan(v) => {
                v.instance_destroy_surface().unwrap();
            },
            _=>{}
        }
    }
    fn resume(&mut self,options: &BackendOptions) {
        match self {
            Backend::Vulkan(v) => {
                match options.render_target {
                    RenderTarget::Headless => {},
                    RenderTarget::Windowed(window_handle,display_handle) => {
                        v.instance_create_surface(window_handle,display_handle).unwrap();
                    }
                }
            },
            _=> {todo!("resume for other backend")}
        }
    }
}

pub enum BackendKind {
    Vulkan,
    OpenGL,
    Headless,
}

// we will start with only one window
pub struct Engine {
    // window: Option<Window>,
    graphics_backend: Backend,
}

impl Engine {
    pub fn new(backend_kind: BackendKind) -> Self {
        // create a vulkan backend, then an opengl backend
        let backend = match backend_kind {
            BackendKind::Vulkan => {
                let v = VulkanBackend::new();
                Backend::Vulkan(v)
            }
            BackendKind::OpenGL => Backend::OpenGL(()),
            BackendKind::Headless => Backend::Headless(()),
        };

        Self {
            graphics_backend: backend,
        }
    }
    pub fn init_backend(&mut self, options: &BackendOptions) -> Result<(), anyhow::Error> {
        self.graphics_backend.init(options)
    }
    pub fn on_suspended(&mut self) {
        self.graphics_backend.suspend();

    }
    pub fn on_resumed(&mut self,options: &BackendOptions) {
        self.graphics_backend.resume(options);
    }
}
