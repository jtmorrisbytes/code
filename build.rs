fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libcode/libcode.c");
    println!("cargo:rerun-if-changed=libcode/libcode.h");
    cc::Build::new().file("libcode/libcode.c")
    .include("./include")
    .compile("libcode")
}