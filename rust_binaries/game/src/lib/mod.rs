pub mod obj;


pub struct Model {
    file: obj::Obj,
}
impl Model {
    pub fn try_load(path:&str) -> Result<Self,Box<dyn std::error::Error>> {
        let obj = obj::parse_obj(path)?;
        Ok(Self{file:obj})
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

const GL_CLEAR_COLOR: (f32,f32,f32,f32) = (0.0,0.0,0.0,1.0);



pub struct Game {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    model: Model,
    frame_count:u64,
    // right now, we only have support for one global vertex shader
    vertex_shader_id: gl::types::GLuint,
    projection_matrix: [f32;16]
}
pub extern "system" fn debug_message_callback(source:u32,ty:u32,id:u32,severity:u32,length:gl::types::GLsizei,msg:*const i8,usrdata:*mut std::ffi::c_void){
    
        let cstr = unsafe {std::ffi::CStr::from_ptr(msg)};
        let message = cstr.to_string_lossy().to_string();
        println!("{source},{ty},{id},{severity},{message}");
    
}

fn get_gl_version_i() -> (i32,i32) {
    let mut major = 0;
    let mut minor = 0;
    unsafe {
        gl::GetIntegerv(gl::MAJOR_VERSION,&mut major);
        gl::GetIntegerv(gl::MINOR_VERSION,&mut minor);
    }
    (major,minor)
}
fn compile_vertex_shader_from_file(shader_index: u32,path: &str) -> Result<(),String> {
    let mut file = std::fs::File::open(path).map_err(|e|e.to_string())?;
    let mut source = String::new();
    file.read_to_string(&mut source).ok();
    unsafe{
            let cstring = std::ffi::CString::new(source).unwrap();
            gl::ShaderSource(shader_index,1,&cstring.as_ptr(),&(cstring.as_bytes_with_nul().len() as i32));
            gl::CompileShader(shader_index);
        }

    // check for errors
    let mut log_length = 0;
    unsafe {
        gl::GetShaderiv(shader_index, gl::INFO_LOG_LENGTH, &mut log_length);
    }
    let mut buf = vec![0i8;10];
    while buf.len() < log_length as usize {
        buf.push(0)
    }
    
    unsafe {
        gl::GetShaderInfoLog(shader_index, buf.len() as i32, &mut log_length, buf.as_mut_ptr());
    }
    let cstr = unsafe {std::ffi::CStr::from_ptr(buf.as_ptr())};
    let error_log = cstr.to_string_lossy().to_string();
    if error_log.len() > 0 {
        return Err(error_log)
    }

    Ok(())
}



impl Game {
    pub fn initialize() -> Self {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
        let (mut window, events) = glfw
            .create_window(800, 600, "a game window", glfw::WindowMode::Windowed)
            .unwrap();
        window.make_current();
        gl::load_with(|name| glfw.get_proc_address_raw(name));
        let (major,minor) = get_gl_version_i();
        if major <2 && minor < 0 {
            panic!("Unsupported opengl version {major}.{minor}. minimum supported version: 2.0")
        }
        unsafe {
            gl::DebugMessageCallback(Some(debug_message_callback), std::ptr::null());
        }
        println!("{}",std::env::current_dir().unwrap().display());
        let model: Model = Model::try_load("teapot.obj").unwrap();

        // set up the vertex shader
        let vertex_shader = unsafe {
            // create a shader object
            gl::CreateShader(gl::VERTEX_SHADER)
        };
        let vertex_shader_program_id = unsafe {
            // create a program object to assocaite with a shader object
            gl::CreateProgram()
        };
        unsafe{
            // associate the shader object with the program object
            gl::AttachShader(vertex_shader_program_id, vertex_shader);
        }
        // compile the vertex shader
        compile_vertex_shader_from_file(vertex_shader, "vertex_shader.glsl").unwrap();
        
        // link the vertex shader
        unsafe {gl::LinkProgram(vertex_shader_program_id);}
        // check for errors
        let mut success: gl::types::GLint = 0;
        unsafe {gl::GetProgramiv(vertex_shader_program_id,gl::LINK_STATUS, &mut success)}
        if success !=1 {
            panic!("Failed to link program")
        }
        unsafe {gl::UseProgram(vertex_shader_program_id);}

        let projection_matrix_shader_location = unsafe {gl::GetUniformLocation(vertex_shader_program_id,c"projection".as_ptr())};
        let projection_matrix = [0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1,0.1];
        unsafe {gl::UniformMatrix4fv(projection_matrix_shader_location,1,gl::FALSE,projection_matrix.as_ptr())}


        Self{glfw,window,events,frame_count:0,model,vertex_shader_id:vertex_shader,projection_matrix}
    }
    fn draw(&mut self) {
        
        unsafe {
            gl::ClearColor(GL_CLEAR_COLOR.0,GL_CLEAR_COLOR.1,GL_CLEAR_COLOR.2,GL_CLEAR_COLOR.3);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, self.window.get_size().0, self.window.get_size().1);
            
        }
        // draw the teapot model

        // upload data to the gpu
        
        // let mut data: Vec<gl::types::GLfloat> = Vec::new();

        // for vert in self.model.file.list_verticies() {
        //     data.push(vert.x());
        //     data.push(vert.y());
        //     data.push(vert.z());
        //     // data.push(1.0);
        //     // data.push(1.0);
        //     // data.push(1.0);
        // }
        let data: [f32;9] = [
        -0.5, -0.5, 0.0, // Bottom-left vertex
         0.5, -0.5, 0.0, // Bottom-right vertex
         0.0,  0.5, 0.0  // Top vertex
        ];
        
        let mut vbo_id = 0;
        let mut vertex_array = 0;
        unsafe {
            // create a vertex buffer object
            gl::GenBuffers(1, &raw mut vbo_id);
            
            // set the buffer as an array buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (data.len() * std::mem::size_of::<gl::types::GLfloat>()) as isize, data.as_ptr().cast(), gl::STATIC_DRAW);
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(0);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,3,gl::FLOAT,0,0,std::ptr::null());
            gl::DrawArrays( gl::TRIANGLES, 0, data.len() as i32);
            gl::Flush();
            gl::DeleteBuffers(1, &vbo_id);
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