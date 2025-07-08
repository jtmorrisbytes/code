// steps to parse a c file.
/*
we need to understand the syntax of the C language just enough to parse OPENGL, WIN32, and X11 Headers.
function declarations, enums,etc.
I want to stray from the way that C parsers typically work in that i dont want to just 'paste' code inside
#include. i want #include to work more like 'import' in more modern systems

// we need to define the language grammar



*/

#[derive(Debug,thiserror::Error)]
pub enum CompilerErrorKind {
    // for includes
    #[error("The specified include directory does not exist")]
    IncludePathDoesNotExist,
    #[error("The rust standard library or OS returned an I/O Error while trying to canonicalize the path provided")]
    IncludePathIOError
    

}
#[derive(Debug,thiserror::Error)]
#[error("Compiler Error: {kind} {reason_or_data}")]
pub struct CompilerError {
    kind: CompilerErrorKind,
    reason_or_data: String
}

/// holds the state for this application
pub struct Compiler {
    include_dirs: Vec<std::path::PathBuf>
}
impl Compiler {
    pub fn initialize() -> Self {
        Self {
            include_dirs: vec![]
        }
    }
    pub fn try_append_include_directory(&mut self,dir: &str) -> Result<(),CompilerError>  {
        let mut path = std::path::PathBuf::from(dir);
        if !path.exists() {
            return Err(CompilerError{kind:CompilerErrorKind::IncludePathDoesNotExist,reason_or_data: path.display().to_string()});
        }
        path = match path.canonicalize() {
            Ok(path)=>path,
            Err(e) => {
                return Err(CompilerError { kind: CompilerErrorKind::IncludePathIOError, reason_or_data: format!("{}",e) })
            }
        };
        self.include_dirs.push(path);

        Ok(())
    }
    pub fn create_translation_unit(&self,filename_or_path: &str) {

    }
}


// a naieve attempt to parse and understand a C header file
pub fn load_and_parse_c_header(state: &Compiler,path: &str) -> () {
    // attempt to load and understand this header file
}
#[cfg(all(test,unix))]
#[test]
pub fn test_parse_GL_headers() -> Result<(),Box<dyn std::error::Error>>{
    let mut compiler = Compiler::initialize();
    
    compiler.try_append_include_directory("/usr/include/GL")?;

    compiler.create_translation_unit("gl.h");

    

    Ok(())
}