fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    // match (target_arch.as_str(),target_os.as_str()) {
    //     ("x86_64","linux") => {
    //         println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/")
    //     },
    //     _=>{
    // println!("cargo:warning={target_arch},{target_os} may not be supported");

    //     }
    // }
    if &target_os == "linux" {
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");
    }
}
