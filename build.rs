extern crate bindgen;
use std::path::PathBuf;
use std::env;


fn main() {
    let libdir_path = PathBuf::from("libg722-vendor")
        .canonicalize()
        .expect("cannot canonicalize path");
    
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib={}", "g722");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("bindgen error");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("couldn't write bindings!");
}