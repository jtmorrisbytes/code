use libgame::engine::{Engine, backend::BackendOptions};
use winit::{application::ApplicationHandler, event::WindowEvent, window::WindowAttributes};

pub struct ExampleGame {
    engine: Engine,
    window: Option<winit::window::Window>,
    backend_options: BackendOptions
}
impl ApplicationHandler for ExampleGame {
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.window.is_none() {
            return;
        }
        let window = self.window.as_ref().unwrap();
        if window.id() != window_id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default();
            let w = event_loop.create_window(window_attributes).unwrap();
            self.window.replace(w);
            self.backend_options = BackendOptions{render_target:libgame::engine::backend::RenderTarget::windowed(self.window.as_ref().unwrap())};
            self.engine
                .init_backend(&self.backend_options)
                .unwrap();
        }
        let window = self.window.as_ref().unwrap();
        self.engine.on_resumed(&self.backend_options);
    }
    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // you must destroy and recreate the surface on suspended and resumed
        self.engine.on_suspended();
    }
}

pub fn main() {
    // we first create the engine and start the backend in headless mode
    let mut engine = Engine::new(libgame::engine::BackendKind::Vulkan);
    let backend_options = BackendOptions {
        render_target: libgame::engine::backend::RenderTarget::Headless,
    };
    engine.init_backend(&backend_options).unwrap();
    let mut game = ExampleGame {
        engine,
        window: None,
        backend_options
    };

    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    event_loop.run_app(&mut game).unwrap();
}
