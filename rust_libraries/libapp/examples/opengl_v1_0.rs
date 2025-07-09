use libapp::gfx::opengl::glx::{GLXVersion, WindowAndDisplayHandle};
use winit::{
    platform::{wayland::EventLoopBuilderExtWayland, x11::EventLoopBuilderExtX11},
    raw_window_handle::{
        DisplayHandle, HasDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle, HasWindowHandle, RawDisplayHandle
    },
    window::WindowAttributes,
};

struct TestApp {
    window: Option<winit::window::Window>,
}
impl winit::application::ApplicationHandler for TestApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // iniitalize the graphics engine (opengl) using winit and raw window handles. opengl requires a window to be created to obtain a context. even if not visible

        // create an opengl context
        self.window.replace(
            event_loop
                .create_window(WindowAttributes::default().with_visible(false))
                .unwrap(),
        );
        let window = self.window.as_ref().unwrap();
        let handle_dyn_object = (Box::new(window) as Box<dyn WindowAndDisplayHandle>);
        

        // request the supported gl version. Im crazy, but I plan to support as low as GL 1.0
        let ctx = libapp::gfx::opengl::glx::glx_create_context(handle_dyn_object).unwrap();
        dbg!(ctx.make_current());
        dbg!(ctx.is_direct());
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // dbg!(window_id,event);
    }
}
pub fn main() {
    let event_loop = winit::event_loop::EventLoop::builder()
        .with_wayland()
        .with_x11()
        .build()
        .unwrap();

    let mut app = TestApp { window: None };
    event_loop.run_app(&mut app).unwrap()
}
