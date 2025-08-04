pub mod actor;
pub mod camera;
pub mod controller;
pub mod cube;
pub mod model;
pub mod obj;
pub mod opengl;
pub mod scene;

use actor::Actor;
use camera::Camera;
use glam::{Mat4, Vec4Swizzles};

// TODO: Once basics of graphics set up, we want to be able to script some code outside of the engine

use std::{io::Read, u64};

use glfw::{Action, Context, Key};

use crate::scene::Scene;
#[derive(Clone)]
pub struct Vec4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
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

const GL_CLEAR_COLOR: (f32, f32, f32, f32) = (0.5, 0.5, 0.5, 1.0);

pub struct Game {
    scene: Scene,
    frame_count: u64,
    // right now, we only have support for one global vertex shader
    vertex_shader: Shader,
    fragment_shader: Shader,
    projection_matrix: [f32; 16],
    view_matrix: [f32; 16],
    // camera: Camera,
}
pub extern "system" fn debug_message_callback(
    _source: u32,
    _ty: u32,
    _id: u32,
    _severity: u32,
    _length: gl::types::GLsizei,
    _msg: *const i8,
    _usrdata: *mut std::ffi::c_void,
) {
    // let cstr = unsafe { std::ffi::CStr::from_ptr(msg) };
    // let message = cstr.to_string_lossy().to_string();
    // println!("{source},{ty},{id},{severity},{message}");
}

pub struct Shader {
    kind: gl::types::GLenum,
    shader_id: gl::types::GLuint,
    program_id: gl::types::GLuint,
}
impl Shader {
    pub fn create_from_file(
        kind: gl::types::GLenum,
        path: &str,
        program_id: Option<gl::types::GLuint>,
    ) -> Result<Self, String> {
        // compile shader on the fly
        let shader_id = unsafe { gl::CreateShader(kind) };
        let program_id = program_id.unwrap_or(unsafe { gl::CreateProgram() });
        unsafe {
            gl::AttachShader(program_id, shader_id);
        }
        opengl::compile_shader_from_file(shader_id, path)
            .map_err(|e| format!("failed to compile {e}"))?;
        unsafe {
            gl::LinkProgram(program_id);
        }
        // check for errors
        let mut success: gl::types::GLint = 0;
        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success) }
        if success != 1 {
            return Err(format!("Failed to link program"));
        }
        return Ok(Self {
            program_id,
            shader_id,
            kind,
        });
    }
    pub fn try_get_uniform_location(&self, name: &str) -> Result<gl::types::GLint, String> {
        let cstr_name = std::ffi::CString::new(name).unwrap();
        let index = unsafe { gl::GetUniformLocation(self.program_id, cstr_name.as_ptr()) };
        if index < 0 {
            return Err(format!(
                "glGetUniformLocation: location for variable {name} not found or is invalid"
            ));
        } else {
            return Ok(index);
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

// all of the 'objects' or 'actors' within the current 'world'

impl Game {
    pub fn initialize() -> Self {
        opengl::gl_clear_color(
            GL_CLEAR_COLOR.0,
            GL_CLEAR_COLOR.1,
            GL_CLEAR_COLOR.2,
            GL_CLEAR_COLOR.3,
        );
        unsafe {
            gl::Enable(gl::PROGRAM_POINT_SIZE);
            gl::Enable(gl::VERTEX_PROGRAM_POINT_SIZE);
            gl::Enable(gl::DEPTH_TEST)
        }

        unsafe {
            gl::DebugMessageCallback(Some(debug_message_callback), std::ptr::null());
        }
        let program_id = unsafe { gl::CreateProgram() };
        let vertex_shader =
            Shader::create_from_file(gl::VERTEX_SHADER, "vertex_shader.glsl", Some(program_id))
                .expect("Vertex shader to be created");
        // // projection matrix
        // let projection_index = vertex_shader
        //     .try_get_uniform_location("projection")
        //     .expect("Valid uniform location for 'projection'");
        const projection_matrix: [gl::types::GLfloat; 16] = [
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ];
        // unsafe { gl::UniformMatrix4fv(projection_index, 1, gl::FALSE, projection_matrix.as_ptr()) }

        // we need to initialize a camera to provide a view matrix
        let camera = Camera::new(
            glam::vec3(1.0, 0.5, 1.0),
            glam::vec3(0.0, 0.0, 0.0),
            glam::vec3(0.0, 1.0, 0.0),
        );

        // let view_matrix_index = vertex_shader
        //     .try_get_uniform_location("view")
        //     .expect("Valid uniform location for 'view'");

        let view_matrix = Mat4::IDENTITY.to_cols_array();
        // unsafe { gl::UniformMatrix4fv(view_matrix_index, 1, gl::FALSE, view_matrix.as_ptr()) }
        let mut scene = Scene::new(camera);
        // let model: Model = Model::try_load("teapot.obj").unwrap();

        let cube = cube::Cube::new(glam::vec3(0.0, 0.0, 0.0), format!("Cube")).unwrap();
        scene.insert_actor(cube);

        // let model_matrix_location = vertex_shader
        //     .try_get_uniform_location("model")
        //     .expect("Valid uniform location for 'model'");
        // const model_matrix: [gl::types::GLfloat; 16] = [
        //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        // ];
        // unsafe { gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, model_matrix.as_ptr()) }

        // fragment shader
        let fragment_shader = Shader::create_from_file(
            gl::FRAGMENT_SHADER,
            "fragment_shader.glsl",
            Some(program_id),
        )
        .expect("Fragment shader to be created");
        // unsafe { gl::UseProgram(fragment_shader.program_id()) }
        unsafe {
            gl::UseProgram(vertex_shader.program_id());
        }
        Self {
            frame_count: 0,
            scene: scene,
            vertex_shader,
            fragment_shader,
            projection_matrix,
            // camera,
            view_matrix: view_matrix,
        }
    }
    pub fn process_keyboard_event(
        &mut self,
        key: glfw::Key,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
        dbg!(key, action, modifiers);
    }
    pub fn render(&mut self) {
        // clear the screen and set the viewport

        opengl::gl_clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        // opengl::gl_viewport(0, 0, self.window.get_size().0, self.window.get_size().1);

        // set the view matrix for this frame

        // upload data to the gpu

        for (_actor_id, actor) in self.scene.actors_iter() {
            let mut data: Vec<gl::types::GLfloat> = Vec::new();
            let rotation = actor.get_rotation();
            let world_space_position = actor.get_position();
            let local_space_origin = actor.get_model().origin();
            for (_vert_id, vertex_4d) in actor.get_model().verticies_iter() {
                data.push(vertex_4d.x);
                data.push(vertex_4d.y);
                data.push(vertex_4d.z);
            }

            // local space position
            let program_state_is_valid =
                opengl::gl_validate_program(self.vertex_shader.program_id()).unwrap_or(false);
            if !program_state_is_valid {
                eprintln!("Warining: vertex shader program is not in a valid state and may not run")
            }

            unsafe {gl::UseProgram(self.vertex_shader.program_id());}
            // only send the uniform if it is present
            if let Ok(uniform_location) = self
                .vertex_shader
                .try_get_uniform_location("local_space_rotation")
            {
                unsafe {
                    gl::Uniform3f(uniform_location, rotation.0, rotation.1, rotation.2);
                    let status = gl::GetError();
                    if status != gl::NO_ERROR {
                        println!("Failed to send local_space_rotation: {status}");
                    }
                }
            }
            // unsafe {
            //     gl::Uniform3f(
            //         self.vertex_shader
            //             .try_get_uniform_location("local_space_origin")
            //             .unwrap(),
            //         local_space_origin.x,
            //         local_space_origin.y,
            //         local_space_origin.z,
            //     );
            // }
            // data.push(1.0);
            // data.push(1.0);
            // data.push(1.0);
            // update view matrix
            // unsafe {
            //     let view_matrix_location =
            //         self.vertex_shader.try_get_uniform_location("view").unwrap();
            //     gl::UniformMatrix4fv(
            //         view_matrix_location,
            //         1,
            //         gl::FALSE,
            //         self.view_matrix.as_ptr(),
            //     )
            // }

            let mut vertex_buffer_object = 0;
            let mut vertex_array_object = 0;
            let mut element_array_object = 0;
            unsafe {
                gl::GenVertexArrays(1, &mut vertex_array_object);
                gl::BindVertexArray(vertex_array_object);
                // create a vertex buffer object
                gl::GenBuffers(1, &raw mut vertex_buffer_object);
                gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (data.len() * std::mem::size_of::<gl::types::GLfloat>()) as isize,
                    data.as_ptr().cast(),
                    gl::STATIC_DRAW,
                );

                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    std::mem::size_of::<gl::types::GLfloat>() as i32,
                    std::ptr::null(),
                );
                gl::EnableVertexAttribArray(0);
                gl::DrawArrays(gl::TRIANGLES, 0, data.len() as i32);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::Flush();
                gl::DeleteBuffers(1, &vertex_buffer_object);
                gl::DeleteBuffers(1, &vertex_array_object);
            }
        }

        self.frame_count = self.frame_count.wrapping_add(1);
    }

    pub fn process_window_event(&mut self, event: &glfw::WindowEvent) {
        for (_, actor) in self.scene.actors_iter_mut() {
            actor.process_window_event(&event);
        }
        match event {
            glfw::WindowEvent::Key(Key::A, _, _, _) => {
                let mut cam_position = self.scene.camera().position();
                cam_position.x = cam_position.x - 0.1;
                self.scene.camera_mut().set_position(cam_position);
            }
            _ => {
                dbg!(event);
            }
        }
    }
    pub fn update(&mut self) {
        for (_, actor) in self.scene.actors_iter_mut() {
            actor.process_physics();
        }
    }
}
