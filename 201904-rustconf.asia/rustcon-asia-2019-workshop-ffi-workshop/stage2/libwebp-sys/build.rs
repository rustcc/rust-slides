use bindgen::Builder;
use cmake::Config;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=png");
    println!("cargo:rustc-link-lib=static=webp");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-search=native=/usr/local/include/webp");
    println!("cargo:rustc-link-search=native=/usr/include/");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/local/opt/zlib/lib");
    println!("cargo:rustc-link-search=native=/usr/local/opt/zlib/include");
    let dst = Config::new("libwebp")
        .build_target("webp")
        .always_configure(true)
        .define("WEBP_BUILD_GIF2WEBP", "OFF")
        .define("WEBP_BUILD_VWEBP", "OFF")
        .define("WEBP_BUILD_IMG2WEBP", "OFF")
        .define("WEBP_BUILD_WEBPMUX", "OFF")
        .define("WEBP_BUILD_DWEBP", "OFF")
        .define("WEBP_BUILD_ANIM_UTILS", "OFF")
        .define("WEBP_BUILD_CWEBP", "OFF")
        .define("WEBP_BUILD_WEBPINFO", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}/build", &dst.display());
    let png_library_path = Path::new("/usr/local/include");
    let zlib_library_path = Path::new("/usr/local/opt/zlib/include");
    cc::Build::new()
        .cpp(true)
        .define("WEBP_HAVE_PNG", None)
        .file("pngwebp/pngdec.c")
        .file("pngwebp/imageio_util.c")
        .file("pngwebp/metadata.c")
        .include(png_library_path)
        .include(zlib_library_path)
        .compile("libpngdec.a");

    let bindings = Builder::default()
        .derive_default(true)
        .header("wrapper.h")
        .header("pngwebp/pngdec.h")
        .header("pngwebp/imageio_util.h")
        .header("pngwebp/metadata.h")
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");
}
