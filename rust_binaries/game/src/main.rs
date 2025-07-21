use glfw::Context;


pub struct Game {
    width:u32,
    height:u32,
}

impl Game {
    pub fn initialize(width:u32,height:u32) -> Self {

        Self{
            width: width.max(1),
            height:height.max(1)
        }
    }
    pub fn resize(&mut self,width:u32,height:u32) {
        self.width = width.max(1);
        self.height = height.max(1);


    }
    pub fn update(&mut self)  {

    }
    pub fn draw(&mut self) {
        // clear buffers
        unsafe {
            gl::ClearColor(0.0,0.0,0.0,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
           
        };
    }
    pub fn shutdown(&mut self) {

    }
}



pub fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
    let (mut window,events) = glfw.create_window(800, 600, "a game window", glfw::WindowMode::Windowed).unwrap();
    window.make_current();

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();
        for (_,event)  in glfw::flush_messages(&events){
            dbg!(event);
        }
    }

}
