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
    pub fn new(backend_kind: BackendKind,render_target:RenderTarget) -> Result<Self,anyhow::Error> {
        // create a vulkan backend, then an opengl backend
        let backend = match backend_kind {
            BackendKind::Vulkan => {
                let v = VulkanBackend::init(render_target)?;
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
