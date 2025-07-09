use std::fmt::format;

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
fn main() {
    let mut args = std::env::args();
    let root_path = args.nth(1).expect("File path as first argument to program");
    let root_path = std::path::PathBuf::from(root_path);
    root_path.canonicalize().expect("Valid file path as first argument");

    dbg!(&root_path);

    // create a directory to contain all of the bindings
    let bindings_dir_path = root_path.join("bindings");
    // clean the directory
    if bindings_dir_path.exists() {
        std::fs::remove_dir_all(&bindings_dir_path).ok();
    }
    std::fs::create_dir_all(&bindings_dir_path).unwrap();

    let bindings_mod_rs = format!(r"pub mod gl;");
    std::fs::write(bindings_dir_path.join("mod.rs"), bindings_mod_rs).ok();

    // create a subdirectory for opengl bindings
    let gl_bindings_dir = bindings_dir_path.join("gl");
    std::fs::create_dir(&gl_bindings_dir).ok();

    let mut bindings_root_mod_rs = String::new();

    // opengl 1.0 - 1.5

    for minor_version in 0..=5 {
        let module_name = format!("v1_{minor_version}");
        let module_file_name = format!("{module_name}.rs");
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(gl_bindings_dir.join(module_file_name))
            .unwrap();
        let registry = Registry::new(
            Api::Gl,
            (1, minor_version),
            Profile::Compatibility,
            Fallbacks::All,
            [],
        );
        registry.write_bindings(StructGenerator, &mut file).unwrap();
        bindings_root_mod_rs = bindings_root_mod_rs + &format!("pub mod {module_name};\n");
    }

    // opengl 2.0-2.1

    for minor_version in 0..=1 {
        let module_name = format!("v2_{minor_version}");
        let module_file_name = format!("{module_name}.rs");
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(gl_bindings_dir.join(module_file_name))
            .unwrap();
        let registry = Registry::new(
            Api::Gl,
            (2, minor_version),
            Profile::Compatibility,
            Fallbacks::All,
            [
                "GL_ARB_shader_objects",
                "GL_ARB_vertex_shader",
                "GL_ARB_fragment_shader",
                "GL_ARB_shading_language_100",
                "GL_ARB_vertex_buffer_object",
                "GL_ARB_pixel_buffer_object",
            ],
        );
        registry.write_bindings(StructGenerator, &mut file).unwrap();
        bindings_root_mod_rs = bindings_root_mod_rs + &format!("pub mod {module_name};\n");
    }

    // opengl 3.0 - 3.1

    for minor_version in 0..=1 {
        let module_name = format!("v3_{minor_version}");
        let module_file_name = format!("{module_name}.rs");
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(gl_bindings_dir.join(module_file_name))
            .unwrap();
        let registry = Registry::new(
            Api::Gl,
            (3, minor_version),
            Profile::Compatibility,
            Fallbacks::All,
            [
                "GL_ARB_shader_objects",
                "GL_ARB_vertex_shader",
                "GL_ARB_fragment_shader",
                "GL_ARB_geometry_shader_4",
                "GL_ARB_texture_float",
                "GL_ARB_framebuffer_object",
                "GL_ARB_vertex_array_object",
            ],
        );
        registry.write_bindings(StructGenerator, &mut file).unwrap();
        bindings_root_mod_rs = bindings_root_mod_rs + &format!("pub mod {module_name};\n");
    }

    // opengl 4.0 - 4.6

    for minor_version in 0..=5 {
        let module_name = format!("v4_{minor_version}");
        let module_file_name = format!("{module_name}.rs");
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(gl_bindings_dir.join(module_file_name))
            .unwrap();
        let registry = Registry::new(
            Api::Gl,
            (4, minor_version),
            Profile::Compatibility,
            Fallbacks::All,
            [
                "ARB_texture_cube_map_array",
                "ARB_texture_gather",
                "ARB_texture_query_lod",
                "ARB_draw_indirect",
                "ARB_gpu_shader5",
                "ARB_gpu_shader_fp64",
                "ARB_tessellation_shader",
                "ARB_vertex_type_2_10_10_10_rev",
                "ARB_transform_feedback3",
            ],
        );
        registry.write_bindings(StructGenerator, &mut file).unwrap();
        bindings_root_mod_rs = bindings_root_mod_rs + &format!("pub mod {module_name};\n");
    }

    let mut wgl_file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(bindings_dir_path.join("wgl.rs"))
            .unwrap();
    let wgl_registry = Registry::new(Api::Wgl, (1,0), Profile::Compatibility, Fallbacks::All, []);
    
    wgl_registry.write_bindings(StructGenerator, &mut wgl_file).unwrap();
    bindings_root_mod_rs= bindings_root_mod_rs + "pub mod wgl;\n";


    // GLX 1.0 - 1.4
    let glx_dir = bindings_dir_path.join("glx");
    std::fs::create_dir(&glx_dir).unwrap();
    for minor_version in 0..=4 {
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(glx_dir.join(format!("v1_{minor_version}.rs")))
            .unwrap();
        let registry = Registry::new(
            Api::Glx,
            (1, minor_version),
            Profile::Compatibility,
            Fallbacks::All,
            [
            ],
        );
        registry.write_bindings(StructGenerator, &mut file).unwrap();
    }
    std::fs::write(gl_bindings_dir.join("mod.rs"), bindings_root_mod_rs).ok();
}
