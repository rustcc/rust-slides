use bindgen::Builder;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=webp");
    println!("cargo:rustc-link-search=native=/usr/local/include/webp");
    println!("cargo:rustc-link-search=native=/usr/include/");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    
    let bindings = Builder::default()
        .derive_default(true)
        .header("wrapper.h")
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");
}
