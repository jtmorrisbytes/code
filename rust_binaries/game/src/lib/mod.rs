pub mod obj;
pub mod gl_utils;



// TODO: Once basics of graphics set up, we want to be able to script some code outside of the engine

#[derive(Clone)]
pub struct Model {
    verticies: std::collections::BTreeMap<u32,Vertex4f>,
}
impl Model {
    pub fn try_load(path:&str) -> Result<Self,Box<dyn std::error::Error>> {
        let obj = obj::parse_obj(path)?;
        Ok(Self{verticies: obj.into_verticies()})
    }
}



use std::{io::Read, u64};

use glfw::Context;
#[derive(Clone)]
pub struct Vertex4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}
impl Vertex4f {
    pub fn new(x:f32,y:f32,z:f32,w:f32) -> Self {
        Self{x,y,z,w}
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
}


pub struct Vertex3f {
    x:f32,
    y:f32,
    z:f32,
}
pub struct Vertex2f {
    x:f32,
    y:f32,
}

const GL_CLEAR_COLOR: (f32,f32,f32,f32) = (0.5,0.5,0.5,1.0);



pub struct Game {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    model: Model,
    frame_count:u64,
    // right now, we only have support for one global vertex shader
    vertex_shader: Shader,
    projection_matrix: [f32;16]
}
pub extern "system" fn debug_message_callback(source:u32,ty:u32,id:u32,severity:u32,length:gl::types::GLsizei,msg:*const i8,usrdata:*mut std::ffi::c_void){
    
        let cstr = unsafe {std::ffi::CStr::from_ptr(msg)};
        let message = cstr.to_string_lossy().to_string();
        println!("{source},{ty},{id},{severity},{message}");
    
}






pub struct Shader{
    kind: gl::types::GLenum,
    shader_id: gl::types::GLuint,
    program_id: gl::types::GLuint
}
impl Shader {
    pub fn create_from_file(kind: gl::types::GLenum,path: &str) -> Result<Self,String> {
        // compile shader on the fly
        let shader_id  = unsafe {gl::CreateShader(kind)};
        let program_id = unsafe {gl::CreateProgram()};
        unsafe {gl::AttachShader(program_id,shader_id);}
        gl_utils::compile_shader_from_file(shader_id,path).map_err(|e| format!("failed to compile {e}"))?;
        unsafe {gl::LinkProgram(program_id);}
            // check for errors
            let mut success: gl::types::GLint = 0;
            unsafe {gl::GetProgramiv(program_id,gl::LINK_STATUS, &mut success)}
            if success !=1 {
                return Err(format!("Failed to link program"))
            }
        return Ok(Self{program_id,shader_id,kind})
    }
    pub fn try_get_uniform_location(&self,name: &str) -> Result<gl::types::GLint,String> {
        let cstr_name = std::ffi::CString::new(name).unwrap();
        let index = unsafe {gl::GetUniformLocation(self.program_id,cstr_name.as_ptr())};
        if index < 0 {
            return Err(format!("glGetUniformLocation: location for variable {name} not found or is invalid"))
        }
        else {
            return Ok(index)
        }
    }
    pub fn kind(&self) -> gl::types::GLenum {
        self.kind
    }
    pub fn shader_id(&self) -> gl::types::GLuint {
        self.shader_id
    }
    pub fn program_id(&self) -> gl::types::GLuint {
        self.program_id
    }

}

// an 'object' inside the game:
// a 'model' that defines the vertex data like colors, textures, etc
// a 'position' in 'world-space'
// todo: rotation




#[derive(Clone)]
pub struct Actor {
    model:Model,
    /// the position in world space
    position: glam::Vec3,
}


// all of the 'objects' or 'actors' within the current 'world'

pub struct Scene {
    actors:std::collections::BTreeMap<u32,Actor>,
    camera: Camera,
}
impl Scene {
    pub const fn origin() -> glam::Vec3 {
        glam::Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}
pub struct Camera {
    /// where is our camera in world space?
    position: glam::Vec3,
    /// what is the camera looking at?
    target: glam::Vec3,
    /// what direction is up, relative to the camera
    up: glam::Vec3,
    /// what direction is right relative to the camera
    right: glam::Vec3,
    // the vector starting at the target pointing to the camera along its Z axis
    direction: glam::Vec3

}
impl Camera {
    pub fn calculate_direction_vector(position:glam::Vec3,target:glam::Vec3) -> glam::Vec3 {
        (position - target).normalize()
    }
    pub fn calculate_right_vector(up: glam::Vec3, direction: glam::Vec3) -> glam::Vec3 {
        // the normalized cross product of the direction vector and the up vector
        up.cross(direction).normalize()
    }
    pub fn calculate_up_vector(direction: glam::Vec3,right: glam::Vec3) -> glam::Vec3 {
        direction.cross(right)
    }
    pub fn new(position:glam::Vec3,target:glam::Vec3,up:glam::Vec3) -> Self {
        let camera_direction = Self::calculate_direction_vector(position, target);
        let camera_right = Self::calculate_right_vector(up,camera_direction);
        let camera_up = Self::calculate_up_vector(camera_direction, camera_right);
        Self{position,target,up:camera_up,right:camera_right,direction:camera_direction}
    }
    pub fn get_rotation_matrix(&self) -> glam::Mat4 {
    glam::mat4(
            glam::vec4(self.right.x,self.right.y,self.right.z,0.0),
            glam::vec4(self.up.x,self.up.y,self.up.z,0.0),
            glam::vec4(self.direction.x,self.direction.y,self.direction.z,0.0),
            glam::vec4(0.0,0.0,0.0,1.0)
        )
    }
    pub fn get_translation_matrix(&self) -> glam::Mat4 {
        glam::mat4(
            glam::vec4(1.0, 0.0, 0.0, -self.position.x),
            glam::vec4(0.0, 1.0, 0.0, -self.position.y),
            glam::vec4(0.0,0.0,1.0,self.position.z),
        glam::vec4(0.0, 0.0, 0.0, 1.0))
    }
    pub fn look_at(&self) -> glam::Mat4 {
        let rotation_matrix = self.get_rotation_matrix();
        let translation_matrix = self.get_translation_matrix();
        rotation_matrix * translation_matrix

    } 
}

impl Game {
    pub fn initialize() -> Self {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3,0));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        let (mut window, events) = glfw
            .create_window(800, 600, "a game window", glfw::WindowMode::Windowed)
            .unwrap();
        window.make_current();
        gl::load_with(|name| glfw.get_proc_address_raw(name));
        unsafe {
            gl::DebugMessageCallback(Some(debug_message_callback), std::ptr::null());
        }
        println!("{}",std::env::current_dir().unwrap().display());
        let model: Model = Model::try_load("teapot.obj").unwrap();

        let vertex_shader = Shader::create_from_file(gl::VERTEX_SHADER,"vertex_shader.glsl").expect("Vertex shader to be created");
        unsafe {gl::UseProgram(vertex_shader.program_id());}

        // projection matrix
        let projection_index = vertex_shader.try_get_uniform_location("projection").expect("Valid uniform location for 'projection'");
        const projection_matrix: [gl::types::GLfloat;16]  = [1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0];
        unsafe {gl::UniformMatrix4fv(projection_index,1,gl::FALSE,projection_matrix.as_ptr())}

        let view_matrix_index = vertex_shader.try_get_uniform_location("view").expect("Valid uniform location for 'view'");
        const view_matrix: [gl::types::GLfloat;16] = [1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0];
        unsafe {gl::UniformMatrix4fv(view_matrix_index,1,gl::FALSE,view_matrix.as_ptr())}

        let model_matrix_location = vertex_shader.try_get_uniform_location("model").expect("Valid uniform location for 'model'");
        const model_matrix: [gl::types::GLfloat;16] = [1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0];
        unsafe {gl::UniformMatrix4fv(model_matrix_location,1,gl::FALSE,model_matrix.as_ptr())}

        // fragment shader
        let fragment_shader = Shader::create_from_file(gl::FRAGMENT_SHADER,"fragment_shader.glsl").expect("Fragment shader to be created");
        unsafe {gl::UseProgram(fragment_shader.program_id())}

        Self{glfw,window,events,frame_count:0,model,vertex_shader,projection_matrix}
    }
    fn draw(&mut self) {
        
        unsafe {
            gl::ClearColor(GL_CLEAR_COLOR.0,GL_CLEAR_COLOR.1,GL_CLEAR_COLOR.2,GL_CLEAR_COLOR.3);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, self.window.get_size().0, self.window.get_size().1);
            
        }
        // draw the teapot model

        // upload data to the gpu
        
        let mut data: Vec<gl::types::GLfloat> = Vec::new();

        for (vertex_id,vert) in self.model.verticies.iter() {
            data.push(vert.x());
            data.push(vert.y());
            data.push(vert.z());
            // data.push(1.0);
            // data.push(1.0);
            // data.push(1.0);
        }
        let mut vbo_id = 0;
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            // create a vertex buffer object
            gl::GenBuffers(1, &raw mut vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (data.len() * std::mem::size_of::<gl::types::GLfloat>()) as isize, data.as_ptr().cast(), gl::STATIC_DRAW);
            
            gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,std::mem::size_of::<gl::types::GLfloat>() as i32,std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::DrawArrays( gl::POINTS, 0, data.len() as i32);
            gl::BindBuffer(gl::ARRAY_BUFFER,0);
            gl::Flush();
            gl::DeleteBuffers(1, &vbo_id);
            gl::DeleteBuffers(1,&vao);
        }


        self.frame_count = self.frame_count.wrapping_add(1);
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.window.swap_buffers();
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                dbg!(event);
            }
            self.draw();
        }
    }
}