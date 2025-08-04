use glam::Vec3;
use super::model::Model;
pub struct Cube {
    position: Vec3,
    name:String,
    model:Model
}
pub trait Actor {
    fn new(initial_position:glam::Vec3,name:String) -> Result<Box<dyn Actor>,String> where Self:Sized;
    fn process_window_event(&mut self, event: &glfw::WindowEvent);
    fn process_physics(&mut self);
    // fn render(&self);
    fn get_model<'b,'a:'b>(&'a self) -> &'b Model;
    fn get_model_mut<'b,'a:'b>(&'a mut self) -> &'b mut Model;
    fn get_name(&self) -> &'_ str;
    fn get_position(&self) -> glam::Vec3;
    fn get_rotation(&self) -> (f32,f32,f32);
}

