use winit::{dpi::PhysicalSize, event::WindowEvent};

use super::Application;


impl winit::application::ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            match self::create_window(self, event_loop) {
                Ok(w) => self.window = Some(w),
                Err(os_error) => {
                    eprintln!("Failed to create root window because of an OSError {os_error}");
                    event_loop.exit();
                }

            }
        }
    }
    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("shutting down")
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
        if window.id() != window_id {
            return;
        }
        match event {
            WindowEvent::Resized(size)  => {
                self.height = size.height;
                self.width = size.width;
            }
            WindowEvent::Focused(focus) => {
                self.has_focus = focus
            }
            WindowEvent::CursorMoved { device_id:_, position } => {
                self.cursor_x = position.x;
                self.cursor_y = position.y;
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            _=>{
                dbg!(event);
            }
            
        }
    }
}

fn create_window(app: &mut Application,event_loop: &winit::event_loop::ActiveEventLoop) -> Result<winit::window::Window, winit::error::OsError> {
    let window_attributes = winit::window::WindowAttributes::default();
    event_loop.create_window(window_attributes)
}