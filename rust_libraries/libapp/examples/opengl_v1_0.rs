use winit::{
    event::WindowEvent, raw_window_handle::{
        DisplayHandle, HasDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle
    }, window::WindowAttributes
};

struct TestApp {
    window: Option<winit::window::Window>,
    context: Option<libapp::gfx::opengl::OpenGL1_0Context>
}
impl winit::application::ApplicationHandler for TestApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // iniitalize the graphics engine (opengl) using winit and raw window handles. opengl requires a window to be created to obtain a context. even if not visible


        // create an opengl context
        self.window.replace(
            event_loop
                .create_window(WindowAttributes::default().with_visible(true))
                .unwrap(),
        );
        let window = self.window.as_ref().unwrap();

        let dyn_ptr = (Box::new(window) as Box<dyn HasDisplayHandle>);
        let display_handle = dyn_ptr.display_handle().unwrap();
        
        let dyn_ptr = (Box::new(window) as Box<dyn HasWindowHandle>);
        let window_handle = dyn_ptr.window_handle().unwrap();
        
        let raw_display_handle = display_handle.as_raw();
        let raw_window_handle = window_handle.as_raw();
        
        let ctx = libapp::gfx::opengl::OpenGL1_0Context::try_new(raw_window_handle, raw_display_handle);
        self.context.replace(ctx);
        let context = self.context.as_ref().unwrap();
        context.make_current();

        


        // request the supported gl version. Im crazy, but I plan to support as low as GL 1.0
    }


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
        match event {
            WindowEvent::RedrawRequested=> {
                self.render()
            }
            _=>{}
        }
    }
}
impl TestApp {
    fn render(&mut self) {
        self.context.as_mut().unwrap().draw();
    }
}
pub fn main() {
    let mut event_loop = winit::event_loop::EventLoop::builder();
    let mut _event_loop= &mut event_loop;
    #[cfg(unix)]{
        use winit::platform::{wayland::EventLoopBuilderExtWayland, x11::EventLoopBuilderExtX11};
        _event_loop = _event_loop.with_wayland()
        .with_x11();
    }

    let event_loop = _event_loop.build()
        .unwrap();

    let mut app = TestApp { window: None, context:None };
    event_loop.run_app(&mut app).unwrap()
}
