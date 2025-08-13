impl Backend {
    fn resume(&mut self,options:&BackendOptions) {
        match self {
            Self::Vulkan(v) => v.resume(options),
            _=>{}
        }
    }
    fn suspend(&mut self) {
        match self {
            Self::Vulkan(v) => v.suspend(),
            _=>{}
        }
    }
}

impl Engine {
    pub fn new(backend_kind: BackendKind,backend_options: &BackendOptions) -> Result<Self,anyhow::Error> {
        // create a vulkan backend, then an opengl backend
        let backend = match backend_kind {
            BackendKind::Vulkan => {
                let v = VulkanBackend::init(backend_options)?;
                Backend::Vulkan(v)
            }
            BackendKind::OpenGL => Backend::OpenGL(()),
            BackendKind::Headless => Backend::Headless(()),
        };

        Ok(Self {
            graphics_backend: backend,
        })
    }
    pub fn on_suspended(&mut self) {
        self.graphics_backend.suspend();
    }
    pub fn on_resumed(&mut self, options: &BackendOptions) {
        self.graphics_backend.resume(options);
    }
    pub fn re_init_backend(&mut self, options: &BackendOptions) -> anyhow::Result<()> {
        self.graphics_backend = match self.graphics_backend {
            Backend::Vulkan(_) => Backend::Vulkan(VulkanBackend::init(&options)?),
            Backend::Headless(_) => Backend::Headless(()),
            Backend::OpenGL(_) => Backend::OpenGL(())
        };
        Ok(())
        
    }
}

pub enum BackendKind {
    Vulkan,
    OpenGL,
    Headless,
}

pub enum Backend {
    Vulkan(super::backends::vulkan::VulkanBackend),
    OpenGL(()),
    Headless(()),
}

pub mod backend;

// we will start with only one window
pub struct Engine {
    // window: Option<Window>,
    graphics_backend: Backend,
}
use crate::{
    backends::vulkan::VulkanBackend,
    engine::backend::{BackendOptions, GraphicsBackend, RenderTarget},
};
