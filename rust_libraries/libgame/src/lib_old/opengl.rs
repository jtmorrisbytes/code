#[inline(always)]
fn gl_clear_error_flag(){
    while gl_get_error() != gl::NO_ERROR{}
}
impl FragmentShader {
    pub fn id(&self) -> u32 {
        0
    }
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            vertex_shader: None,
            fragment_shader: None
        }
    }
    pub fn with_vertex_shader(self,shader: VertexShader)->Self{
        Self {vertex_shader: Some(shader),fragment_shader:self.fragment_shader}
    }
    pub fn with_fragment_shader(self,shader:FragmentShader) -> Self{
        Self {fragment_shader: Some(shader),vertex_shader:self.vertex_shader}

    }
    pub fn link(self) -> Result<Program,gl::types::GLenum> {
        let program_id = gl_create_program();

        if let Some(vertex_shader) = self.vertex_shader {
            gl_attach_shader(program_id,vertex_shader.id())?;
        }
        if let Some(fragment_shader) = self.fragment_shader {
            gl_attach_shader(program_id,fragment_shader.id())?;
        }
        gl_link_program(program_id);
        todo!()

    }
}


impl Program {
    pub fn r#use(){}
}impl VertexShader {
    pub fn id(&self) -> u32 {
        0
    }
}


// TODO: check for just in time shader compiler support
#[derive(Debug,thiserror::Error)]
pub enum GLGetValueError {
    #[error("GLInvalidValue")]
    InvalidValue,
    #[error("GLInvalidOperation")]
    InvalidOperation
}

pub enum GLShaderType {
    GlVertexShader,
    GlFragmentShader
}

#[derive(thiserror::Error,Debug)]
pub enum GetShaderValueError{ 
#[error("Get Shader Value Invalid Value")]
    InvalidValue,
#[error("Get Shader Value Invalid Operation")]

    InvalidOperation
}
pub enum GetShaderValue {
    GLShaderType,
    GLDeleteStatus,
    GLCompileStatus,
    GLInfoLogLength,
    GLShaderSourceLength
}




pub fn compile_shader_from_file(shader_index: u32,path: &str) -> Result<(),String> {
    // check for shader compiler support. 
    if !gl_does_support_shader_compiler()? {
        panic!("compiling shaders at runtime is not supported");
    }
    if !gl_is_valid_shader(shader_index) {
        return Err("The shader name {shader_index} does not refer to a valid shader object".to_string())
    }
    
    let mut file = std::fs::File::open(path).map_err(|e|e.to_string())?;
    let mut source = String::new();
    file.read_to_string(&mut source).ok();
    unsafe{
            let cstring = std::ffi::CString::new(source).unwrap();
            gl::ShaderSource(shader_index,1,&cstring.as_ptr(),&(cstring.as_bytes_with_nul().len() as i32));
        }
    if gl_compile_shader(shader_index) != gl::NO_ERROR {
        return Err("Failed to call gl_compile_shader".to_string())
    };

    // check for errors
    if !gl_get_shader_compile_status(shader_index)? {
        let shader_log = gl_get_shader_info_log(shader_index)?;
        return Err(shader_log);
    }


    Ok(())
}

pub fn get_gl_version_i() -> Result<(i32,i32),GLGetVersionIError> {
    let mut major = 0;
    let mut minor = 0;
    
    major = gl_get_integer_v(gl::MAJOR_VERSION).map_err(|e| GLGetVersionIError{message:format!("Failed to get GL_MAJOR_VERSION: {e}")})?;

    minor = gl_get_integer_v(gl::MINOR_VERSION).map_err(|e| GLGetVersionIError{message:format!("Failed to get GL_MINOR_VERSION: {e}")})?;
    Ok((major,minor))
}


pub fn gl_attach_shader(program: gl::types::GLuint,shader: gl::types::GLuint) -> Result<(),gl::types::GLenum> {
    if !gl_is_valid_shader(shader) {
        return Err(gl::INVALID_VALUE);
    }
    if !gl_is_program(program) {
        return Err(gl::INVALID_VALUE);
    }
    gl_clear_error_flag();
    unsafe {
        gl::AttachShader(program, shader);
    }
    let status = gl_get_error();
    if status != gl::NO_ERROR {
        Err(status)
    }
    else {
        Ok(())
    }
}

pub fn gl_clear(flags: gl::types::GLbitfield) -> gl::types::GLenum {
    gl_clear_error_flag();
    unsafe {gl::Clear(flags)}
    return gl_get_error();
}
pub fn gl_clear_color(r:gl::types::GLfloat,g:gl::types::GLfloat,b:gl::types::GLfloat,a:gl::types::GLfloat) {
    unsafe {gl::ClearColor(r,g,b,a)};
}


/// wrapper function for gl::glCompileShader
pub fn gl_compile_shader(shader_index:gl::types::GLuint) -> gl::types::GLenum {
    // clear the error flag
    gl_clear_error_flag();
    unsafe {
        gl::CompileShader(shader_index);
    }
    gl_get_error()


}


pub fn gl_create_program() -> gl::types::GLuint {
    let program_id = unsafe {
        gl::CreateProgram()
    };

    program_id
}


/// checks the gl implementation  for shader compiler support assumes false if support cannot be determined
pub fn gl_does_support_shader_compiler() -> Result<bool,String> {
    gl_get_boolean_v(gl::SHADER_COMPILER).map_err(|e| format!("Failed to get value GL_SHADER_COMPILER while checking if shader compiler is supported: {e}"))
}


pub fn gl_get_boolean_v(pname: gl::types::GLenum) -> Result<bool,GLGetValueError> {
    // clear the error flag
    gl_clear_error_flag();
    // call GetBooleanv

    let mut param: gl::types::GLboolean= 0;

    unsafe {gl::GetBooleanv(pname,&mut param)}
    // check for errors
    match gl_get_error() {
        gl::NO_ERROR => Ok(param !=0),
        gl::INVALID_VALUE => Err(GLGetValueError::InvalidValue),
        gl::INVALID_OPERATION => Err(GLGetValueError::InvalidOperation),
        unknown=> panic!("gl_get_boolean_v: unknown gl error: {unknown:x}")
    }
}



/// wrapper function for glGetError()
pub fn gl_get_error() -> gl::types::GLenum {
    unsafe {gl::GetError()}
}


pub fn gl_get_integer_v(pname: gl::types::GLenum) -> Result<gl::types::GLint,GLGetValueError> {
    let mut param: gl::types::GLint = 0;
    // clear the error flag
    gl_clear_error_flag();
    // call GetIntegerv
    unsafe {gl::GetIntegerv(pname,&mut param)};
    // check for errors
    let status = gl_get_error();
    match status {
        gl::NO_ERROR => Ok(param),
        gl::INVALID_VALUE => Err(GLGetValueError::InvalidValue),
        gl::INVALID_OPERATION => Err(GLGetValueError::InvalidOperation),
        _=> panic!("gl_get_integer_v: unknown gl error: {status:x}")
    }
}

pub fn gl_get_program_iv(program:u32,pname: gl::types::GLenum) -> Result<gl::types::GLint,gl::types::GLenum> {
    gl_clear_error_flag();
    let mut output: gl::types::GLint = 0;
    unsafe {
        gl::GetProgramiv(program, pname, &mut output);
    }
    let status = gl_get_error();
    match status {
        gl::NO_ERROR => Ok(output),
        other => Err(other)
    }

}

/// checks whether or not the shader compiled succesfully
pub fn gl_get_shader_compile_status(shader_index: gl::types::GLuint) -> Result<bool,String> {
    match gl_get_shader_iv(shader_index,GetShaderValue::GLCompileStatus) {
        Ok(gl_int) => Ok(gl_int !=0),
        Err(e) => Err(format!("failed to glGet(GL_COMPILE_STATUS) for shader {shader_index}: {e}"))
        
    }
}

// returns GL_TRUE if shader is flagged for deletion, otherwide GL_FALSE
pub fn gl_get_shader_delete_status(shader_index: gl::types::GLuint) -> Result<bool,String> {
    match gl_get_shader_iv(shader_index,GetShaderValue::GLDeleteStatus) {
        Ok(gl_int) => Ok(gl_int !=0),
        Err(e) => Err(format!("failed to glGet(GL_DELETE_STATUS) for shader {shader_index}: {e}"))
        
    }
}
// gets the program info log





pub fn gl_get_shader_info_log(shader_index: gl::types::GLuint) -> Result<String,String> {
    // check if the shader argument is a valid shader object
    if !gl_is_valid_shader(shader_index) {
        return Err(format!("The shader name {shader_index} is not a vaild shader object"))
    }
    // try to get the info log length
    let log_length = gl_get_shader_info_log_length(shader_index).map_err(|e| format!("Feiled to get the shader info log length: {e}"))?;
    // create a buffer to hold the c-string
    let mut buffer: Vec<gl::types::GLchar> = Vec::new();
    for _ in 0..log_length {
        buffer.push(0)
    }


    gl_clear_error_flag();
    unsafe {gl::GetShaderInfoLog(shader_index,buffer.len().try_into().unwrap_or(gl::types::GLsizei::MAX),std::ptr::null_mut(),buffer.as_mut_ptr())};
    let status = gl_get_error();
    if status != gl::NO_ERROR {
        return Err(format!("Failed to get the shader info log for shader {shader_index}: {status:x}"))
    }
    // this may cause Undefined behavior if the string is null terminated
    let c_str = unsafe {std::ffi::CStr::from_ptr(buffer.as_ptr())};
    let mut string = c_str.to_str().expect("Valid Utf-8 encoded string").to_string();   
    // get the length of the log
    Ok(string)
}
// gets the length of the shader info log. if no error, returns 0
pub fn gl_get_shader_info_log_length(shader_index: gl::types::GLuint) -> Result<gl::types::GLint,String> {
    match gl_get_shader_iv(shader_index,GetShaderValue::GLInfoLogLength) {
        Ok(gl_int) => Ok(gl_int),
        Err(e) => Err(format!("failed to glGet(GL_INFO_LOG_LENGTH) for shader {shader_index}: {e}"))
        
    }
}
/// wrapper function for glGetShaderiv. this function will fail if the opengl implementation does not support shader compilers. if so, you will have to fall back to a pre compiled shader
pub fn gl_get_shader_iv(shader_index: gl::types::GLuint,value_type: GetShaderValue) -> Result<gl::types::GLint,GetShaderValueError> {
    if !gl_is_valid_shader(shader_index) {
        eprintln!("The shader name {shader_index} is not a vaild shader");
        return Err(GetShaderValueError::InvalidValue)
    }
    let mut data: gl::types::GLint = 0;
    gl_clear_error_flag();
    let pname = match value_type {
        GetShaderValue::GLShaderType => gl::SHADER_TYPE,
        GetShaderValue::GLDeleteStatus => gl::DELETE_STATUS,
        GetShaderValue::GLInfoLogLength => gl::INFO_LOG_LENGTH,
        GetShaderValue::GLShaderSourceLength => gl::SHADER_SOURCE_LENGTH,
        GetShaderValue::GLCompileStatus => gl::COMPILE_STATUS
    };
    unsafe {gl::GetShaderiv(shader_index,pname,&mut data)}
    match  gl_get_error() {
        gl::NO_ERROR => Ok(data),
        gl::INVALID_VALUE => Err(GetShaderValueError::InvalidValue),
        gl::INVALID_OPERATION => Err(GetShaderValueError::InvalidOperation),
        unknown_error => panic!("Unknown error returned from glGetShaderiv: {unknown_error:x} for shader {shader_index}")
    }

}
pub fn gl_get_shader_type(shader_index: gl::types::GLuint) -> Result<GLShaderType,String> {
    if !gl_is_valid_shader(shader_index) {
        return Err(format!("The shader name {shader_index} is not a vaild shader"))
    }
    match gl_get_shader_iv(shader_index,GetShaderValue::GLShaderType).map(|gl_int| std::convert::TryInto::<gl::types::GLuint>::try_into(gl_int).expect("Failed to convert GLint to GLuint")) {
        Ok(gl_int) if gl_int == gl::VERTEX_SHADER => Ok(GLShaderType::GlVertexShader),
        Ok(gl_int) if gl_int == gl::FRAGMENT_SHADER => Ok(GLShaderType::GlFragmentShader),
        Ok(gl_int) => {
            panic!("Unknown Value {gl_int} for glGet(GL_SHADER_TYPE) for shader {shader_index}")
        }
        Err(e) => {
            Err(format!("Failed to glGet(GL_SHADER_TYPE): {e}"))
        }
    }

}

pub fn gl_is_program(program: gl::types::GLuint) -> bool {
    unsafe {gl::IsProgram(program)!= gl::FALSE} 
}

// checks if the shader argument is a valid shader object. if any error occurs, returns false
pub fn gl_is_valid_shader(shader: gl::types::GLuint) -> bool {
    unsafe {gl::IsShader(shader)!= gl::FALSE}
}
pub fn gl_link_program(program: gl::types::GLuint) {

    if !gl_is_program(program) {
        panic!("Invalid shader program {program}")
    }
    unsafe {
        gl::LinkProgram(program);
    }
}

pub fn gl_use_program(program_id:u32) {
    
    unsafe {
        gl::ValidateProgram(program_id);
        gl::UseProgram(program_id)
    }
}
pub fn gl_validate_program(program:u32) -> Result<bool,gl::types::GLenum> {
    gl_clear_error_flag();
    unsafe {
        gl::ValidateProgram(program);
    }
    let call_status = gl_get_error();
    if call_status != gl::NO_ERROR {
        return Err(call_status)
    }
    let validate_status = gl_get_program_iv(program, gl::VALIDATE_STATUS)?;
    Ok(validate_status as u8 != gl::FALSE)
}
pub fn gl_viewport(x:gl::types::GLint,y:gl::types::GLint,width:gl::types::GLsizei,height:gl::types::GLsizei) {
    debug_assert!(width >= 0);
    debug_assert!(height >= 0);
    gl_clear_error_flag();
    unsafe {gl::Viewport(x,y,width,height)}
}

pub struct FragmentShader {
    
}
#[derive(thiserror::Error,Debug)]
#[error("An error occured while trying to get the OpenGL Version: {message} ")]
pub struct GLGetVersionIError{
    message:String
}


pub struct ProgramBuilder{
    vertex_shader:Option<VertexShader>,
    fragment_shader:Option<FragmentShader>
}



pub struct Program {}

pub struct VertexShader;

use gl::types;
// We are targeting OpenGL 3.0

// --- GL 3.0 Wrapper functions

use std::io::Read;
