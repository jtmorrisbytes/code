pub fn main()->Result<(),Box<dyn std::error::Error>> {
    print!("cargo:rustc-lib-search=native=/lib/x86_64-linux-gnu");
    print!("cargo:rustc-lib-search=native=/lib/i386-linux-gnu");
    print!("cargo:rustc-link-lib=X11");
    print!("cargo::rustc-link-arg-tests=Append=-lX11");
    print!("cargo::rustc-link-arg-tests=Append=-L /lib/i386-linux-gnu");


    print!("cargo:rerun-if-changed=src/bindings.h");
    print!("cargo:rerun-if-changed=/usr/include/X11/X.h");
    print!("cargo:rerun-if-changed=build.rs");
    



    let bindings = bindgen::builder().header(std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("src").join("bindings.h").to_string_lossy()).generate()?;
    
    
    
    let path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(path)?;
    Ok(())


}