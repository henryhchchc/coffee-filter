use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=JAVA_HOME");

    if let Ok(java_home) = env::var("JAVA_HOME") {
        let platform_include = if cfg!(target_os = "macos") {
            "darwin"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            panic!("Unsupported platform")
        };
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .generate_block(true)
            .prepend_enum_name(false)
            .clang_arg(format!("-I{}/include", java_home))
            .clang_arg(format!("-I{}/include/{}", java_home, platform_include))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        println!(
            "cargo:warning=JAVA_HOME is not set, skipping bindings generation. The build may fail."
        )
    }
}
