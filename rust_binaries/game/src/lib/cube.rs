use glam::Vec3;

use crate::{actor::Actor, model::Model};

pub struct Cube {
    position: Vec3,
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,

    name:String,
    model:Model
}


impl Actor for Cube {
    fn new(initial_position: glam::Vec3,name: String) -> Result<Box<dyn Actor>,String> {
        let model = Model::try_load("cube.obj").map_err(|e|e.to_string())?;
        Ok(Box::new(Self {
            position:initial_position,name,model,
            x_rotation: 0.0_f32.to_degrees(),
            y_rotation: 0.0_f32.to_degrees(),
            z_rotation: 0.0_f32.to_degrees(),
        }))
    }
    fn get_model<'b,'a:'b>(&'a self) -> &'b Model {
        &self.model
    }
    fn get_model_mut<'b,'a:'b>(&'a mut self) -> &'b mut Model {
        &mut self.model
    }
    fn get_name(&self) -> &'_ str {
        &self.name
    }
    fn get_position(&self) -> glam::Vec3 {
        self.position
    }
    fn process_physics(&mut self) {
        // self.position.y = self.position.y - 0.001;
        // self.position.x = self.position.x - 0.001;
        // self.position.z = self.position.z - 0.001;
        if self.x_rotation > 360.0000000 {
            self.x_rotation = 0.0_f32.to_degrees();
        }
        self.x_rotation = self.x_rotation + 0.0000001_f32.to_degrees();

        if self.y_rotation > 360.0000000 {
            self.y_rotation = 0.0_f32.to_degrees();
        }
        self.y_rotation = self.y_rotation + 0.02;
        if self.z_rotation > 359.999999 {
            self.z_rotation = 0.0_f32.to_degrees();
        }
        self.z_rotation = self.z_rotation + 0.015_f32.to_degrees();
        
    }
    fn process_window_event(&mut self, event: &glfw::WindowEvent) {
        
    }
    fn get_rotation(&self) -> (f32,f32,f32) {
        (self.x_rotation,self.y_rotation,self.z_rotation)
    }
}



