#[cfg(target_os="linux")]
pub mod linux_sys;
#[cfg(target_os="windows")]
pub mod windows_sys;

#[cfg(target_os="linux")]
pub use linux_sys as sys;

#[cfg(target_os="windows")]
pub use windows_sys as sys;

use sys::{LibraryInfo,Library,Symbol,Architecture};

#[cfg(target_os="windows")]
// this mod prefixed with libecode to prevent namespace collisions
mod libcode_windows {
    use super::{LibraryInfo,Library,Symbol};
    pub(crate) fn convert_libname_to_shared_library_filename(s:&str) -> String {
        format!("{s}.dll")
    }
    pub(crate) fn find_shared_library(name:&str,architechure:Architecture)->Vec<LibraryInfo>{
        eprintln!("Warning: Not implemented yet!");
        Vec::new()
    }
    pub(crate) fn load_shared_library(library_info: &LibraryInfo) -> Result<Library,String> {
        todo!()
    }
    pub(crate) fn load_symbol(library: Library) -> Result<Symbol,String> {
        todo!()
    }
}

#[cfg(target_os="linux")]
mod libcode_linux {
    use super::Symbol;
    // used with find_shared_libary to determine which library
    /// rust wrapper around dlerror;
    use crate::linux_sys::rust_dlerror;

    use super::{LibraryInfo,Library};

        const ARCH_x86_64:&str = "x86-64";
        const ARCH_i386:&str = "i386";


    pub(crate) fn convert_libname_to_shared_library_filename(s:&str) -> String {
        format!("lib{s}.so")
    }
    pub(crate) fn find_shared_library(name: &str,architechure: super::Architecture) ->Vec<LibraryInfo>{

 


        // run
        let output: std::process::Output = std::process::Command::new("ldconfig").args(&["-p"]).output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if (!output.status.success()) {
            println!("{}",stdout.to_string());
            eprintln!("{}",stderr.to_string());
            panic!("Failed to run ldconfig!")
        }
        let lines:Vec<String> = stdout.split("\n").map(|s| s.replace("\t","") ).collect();
         // left is libname, middle is src and arch, right is libpath

        let libname = self::convert_libname_to_shared_library_filename(name);

        
        let canidates: Vec<LibraryInfo> = lines.iter().map(
            |s| {
                let mut split = s.split(" => ");
                let left = split.next().unwrap_or_default();
                let libpath = split.next().unwrap_or_default();

                let mut split2 = left.split(" ");
                let libname = split2.next().unwrap_or_default();

                let src_arch= split2.next().unwrap_or_default();
                
                let mut split3 = src_arch.split(",");
                let src = split3.next().unwrap_or_default().replace("(","").replace(")","");
                let mut arch = split3.next().unwrap_or_default().replace(")","");

                // if arch is empty, the architechure is assumed to be i386 (x86)
                if arch.len() == 0 {arch = "i386".to_string();}

                let arch =  match arch.as_str() {
                    ARCH_x86_64 => super::Architecture::x86_64,
                    ARCH_i386  => super::Architecture::i386,
                    _=>{unimplemented!()}
                };
                LibraryInfo{
                    name: libname.to_string(),
                    source:src.to_string(),
                    arch:arch,
                    libpath:std::path::PathBuf::from(libpath)
                }
            }
        )
        .filter(|library| {
            library.name.starts_with(&libname) 
        })
        .filter(|library| {
            library.arch == architechure
            // arch.as_str() == architechure
        })
        .collect();
        // let canidates: Vec<(String,String,String,String)> = lines.into_iter().collect();

        canidates
    }
    pub const RTLD_NOW: std::ffi::c_int = 0x00002;
    pub(crate) fn load_shared_library(library_info: &LibraryInfo) -> Result<Library,String> {
        let libpath = library_info.libpath.display().to_string();
        let mut libpath = std::ffi::CString::new(libpath).map_err(|e| format!("Failed to convert library path into a cstring for dlopen operation: {e}"))?;
        let ptr = unsafe {crate::linux_sys::dlopen(libpath.as_ptr(),RTLD_NOW)};
        let _ = crate::linux_sys::rust_dlerror()?;
        if ptr.is_null() {
            return Err("Recieved Null pointer from dlopen".to_string());
        }
        if !ptr.is_aligned() {
            return Err("dlopen pointer not aligned".to_string());
        }

        dbg!(&ptr);

        Ok(Library::new(ptr))
    }

    impl Symbol {
    }
}



pub fn find_shared_library(name: &str,arch: Architecture) -> Vec<LibraryInfo> {
    #[cfg(target_os="windows")]{
       libcode_windows::find_shared_library(name,arch)
    }
    #[cfg(target_os="linux")]{
        libcode_linux::find_shared_library(name,arch)
    }
    #[cfg(all(not(target_os="windows"),not(target_os="linux")))]{
        compile_error!("Unsupported on this platform")
    }
}

pub fn load_shared_library(libraryinfo: &LibraryInfo) -> Result<Library,String> {
        #[cfg(target_os="windows")]{
       libcode_windows::load_shared_library(libraryinfo)
    }
    #[cfg(target_os="linux")]{
        libcode_linux::load_shared_library(libraryinfo)
    }
    #[cfg(all(not(target_os="windows"),not(target_os="linux")))]{
        compile_error!("Unsupported on this platform")
    }
}
pub unsafe fn get_symbol<PtrType>(library: &Library,name: &str) -> Result<PtrType,String> {
        #[cfg(target_os="windows")]{
       libcode_windows::get_symbol(library,name)
    }
    #[cfg(target_os="linux")]{
       unsafe {crate::linux_sys::rust_dlsym::<PtrType>(library,name)}
    }
    #[cfg(all(not(target_os="windows"),not(target_os="linux")))]{
        compile_error!("Unsupported on this platform")
    }
}
unsafe extern "C" {
    pub fn hello() -> std::ffi::c_int;
}