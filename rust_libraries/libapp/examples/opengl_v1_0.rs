use libapp::gfx::opengl::glx::{GLXVersion};
use winit::{
    event::WindowEvent, platform::{wayland::EventLoopBuilderExtWayland, x11::EventLoopBuilderExtX11}, raw_window_handle::{
        DisplayHandle, HasDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle
    }, window::WindowAttributes
};

struct TestApp {
    window: Option<winit::window::Window>,
    context: Option<libapp::gfx::opengl::glx::GLXContext>
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
        
        let ctx = match (raw_display_handle,raw_window_handle) {
            (RawDisplayHandle::Xlib(display_handle),RawWindowHandle::Xlib(window_handle)) => {
                libapp::gfx::opengl::glx::glx_create_context_from_winit_xlib_handle(display_handle, window_handle).unwrap()

            },
            unimpl=>{
                unimplemented!("{unimpl:?}")
            }
        };
        dbg!(ctx.make_current());
        dbg!(ctx.is_direct());
        dbg!(ctx.get_gl_version());
        self.context.replace(ctx);

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
    let event_loop = winit::event_loop::EventLoop::builder()
        .with_wayland()
        .with_x11()
        .build()
        .unwrap();

    let mut app = TestApp { window: None, context:None };
    event_loop.run_app(&mut app).unwrap()
}
