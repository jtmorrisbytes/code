pub mod types {
    use std::ffi::{c_uint,c_uchar,c_int,c_char,c_void,c_ushort,c_float,c_double};
    pub type GLenum = c_uint;
    pub type GLboolean = c_char;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLint = c_int;
    pub type GLubyte = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;

}
// boolean values
const GL_FALSE: types::GLenum = 0;
const GL_TRUE: types::GLenum = 1;

const GL_BYTE
