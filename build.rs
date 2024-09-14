use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{env, io::Write};

use anyhow::{bail, Context};
use regex::bytes::Regex;

static UINT_TY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"pub type ([\w_]+) = ::std::os::raw::c_uint;").expect("The regex is incorrect")
});

fn main() -> Result<(), anyhow::Error> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=JAVA_HOME");

    let Ok(java_home) = env::var("JAVA_HOME") else {
        println!(
            "cargo:warning=\
            JAVA_HOME is not set, skipping bindings generation. The build may fail."
        );
        return Ok(());
    };

    let platform_include = if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        bail!("Unsupported platform")
    };
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_block(true)
        .prepend_enum_name(false)
        .clang_arg(format!("-I{}/include", java_home))
        .clang_arg(format!("-I{}/include/{}", java_home, platform_include))
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .context("Generating bindings")?;

    let buf = {
        let mut buf = Vec::new();
        let mut writer = BufWriter::new(&mut buf);
        bindings
            .write(Box::new(&mut writer))
            .context("Writing bindings")?;
        drop(writer);
        buf
    };

    let bindings_rust_code =
        UINT_TY_REGEX.replace_all(&buf, b"pub type $1 = ::std::os::raw::c_int;");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let binding_dot_rs = File::create(out_path.join("bindings.rs"))?;
    let mut writer = BufWriter::new(binding_dot_rs);
    writer
        .write_all(&bindings_rust_code)
        .context("Writing to bindings.rs")
}
