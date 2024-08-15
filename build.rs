extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Unknown target OS");
    let (link_lib, mut include_path) = match target_os.as_str() {
        "windows" => ("xiapi64", "C:/XIMEA/API/xiAPI".to_string()),
        "linux" => ("m3api", "/opt/XIMEA/include".to_string()),
        "macos" => (
            "m3api",
            "/Library/Frameworks/m3api.framework/Headers".to_string(),
        ),
        x => panic!("Unknown platform: {x}"),
    };

    if let Ok(override_include_path) = env::var("XIAPI_INCLUDE_DIR") {
        include_path = override_include_path;
    };

    println!("cargo:rerun-if-changed=wrapper.h");

    if target_os.as_str() == "macos" {
        println!("cargo:rustc-link-search=framework=/Library/Frameworks");
        println!("cargo:rustc-link-lib=framework=m3api");
    } else {
        println!("cargo:rustc-link-lib={}", link_lib);
    }

    if target_os.as_str() == "windows" {
        println!("cargo:rustc-link-search={}", include_path);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path))
        .constified_enum_module(".*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
