// We are targeting OpenGL 3.0

// --- GL 3.0 Wrapper functions

use std::io::Read;


/// wrapper function for glGetError()
pub fn gl_get_error() -> gl::types::GLenum {
    unsafe {gl::GetError()}
}


// TODO: check for just in time shader compiler support

pub fn gl_get_integer_v(pname: gl::types::GLenum,params: &mut gl::types::GLint) -> Result<(),gl::types::GLenum> {
    // clear the error flag
    let _ = gl_get_error();
    // call GetIntegerv
    unsafe {gl::GetIntegerv(pname,params)}
    // check for errors
    let error = gl_get_error();
    if error == gl::NO_ERROR {
        return Ok(())
    }
    else {
        return Err(error)
    }
}


pub fn gl_get_boolean_v(pname: gl::types::GLenum,params: &mut gl::types::GLboolean) -> Result<(),gl::types::GLenum> {
    // clear the error flag
    let _ = gl_get_error();
    // call GetBooleanv
    unsafe {gl::GetBooleanv(pname,params)}
    // check for errors
    let error = gl_get_error();
    if error == gl::NO_ERROR {
        return Ok(())
    }
    else {
        return Err(error)
    }
}


pub fn get_gl_version_i() -> (i32,i32) {
    let mut major = 0;
    let mut minor = 0;
    gl_get_integer_v(gl::MAJOR_VERSION, &mut major).expect("Failed to get GL_MAJOR_VERSION");
    gl_get_integer_v(gl::MINOR_VERSION, &mut minor).expect("Failed to get GL_MINOR_VERSION");
    (major,minor)
}


/// checks the gl implementation  for shader compiler support assumes false if support cannot be determined
pub fn gl_does_support_shader_compiler() -> bool {
    let mut is_supported: gl::types::GLboolean = 0;
    let result = gl_get_boolean_v(gl::SHADER_COMPILER,&mut is_supported);
    match result {
        Ok(()) => {return is_supported !=0;},
        Err(e) => {
            #[cfg(debug_assertions)]
            println!("Failed to check for compiler support: glGetBooleanv returned an error: GLenum {e:x}");
            return false
        }
    }
}


/// wrapper function for glGetShaderiv. this function will fail if the opengl implementation does not support shader compilers. if so, you will have to fall back to a pre compiled shader
pub fn gl_get_shader_iv(shader_index: gl::types::GLuint,pname: gl::types::GLenum,data: &mut gl::types::GLint) -> Result<(),gl::types::GLenum> {
    let _ = gl_get_error();
    unsafe {gl::GetShaderiv(shader_index,pname,data)}
    let error = gl_get_error();
    if error == gl::NO_ERROR {
        return Ok(())
    }
    else {
        return Err(error)
    }
    
}



pub fn get_shader_info_log(shader_index: gl::types::GLuint) -> Option<String> {
    let mut len = 0;
    // get the length of the log
    todo!()
}



/// wrapper function for gl::glCompileShader
pub fn gl_compile_shader(shader_index:gl::types::GLuint) -> Result<(),gl::types::GLenum> {
    // clear the error flag
    let _ = gl_get_error();
    unsafe {
        gl::CompileShader(shader_index);
    }
    let error = gl_get_error();
    if error != gl::NO_ERROR {
        return Err(error)
    }
    else {
        return Ok(())
    }

}




pub fn compile_shader_from_file(shader_index: u32,path: &str) -> Result<(),String> {
    // check for shader compiler support. 
    if !gl_does_support_shader_compiler() {
        panic!("compiling shaders at runtime is not supported");
    }
    
    let mut file = std::fs::File::open(path).map_err(|e|e.to_string())?;
    let mut source = String::new();
    file.read_to_string(&mut source).ok();
    unsafe{
            let cstring = std::ffi::CString::new(source).unwrap();
            gl::ShaderSource(shader_index,1,&cstring.as_ptr(),&(cstring.as_bytes_with_nul().len() as i32));
        }
    gl_compile_shader(shader_index).map_err(|e| format!("Failed to compile shader: GLenum {e:#x}"))?;

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
