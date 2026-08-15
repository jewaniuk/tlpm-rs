use std::env;
use std::path::PathBuf;

fn main() {
    // check the environment variable for the path to the TLPM include directory
    // otherwise resort to default expected path
    let include_dir = env::var("TLPM_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(r"C:\Program Files\IVI Foundation\VISA\Win64\Include"));
    let header_path = include_dir.join("TLPMX.h");
    if !header_path.exists() {
        panic!(
            "could not find 'TLPMX.h' at '{}', please install the Thorlabs Optical Parameter Monitor software or set the 'TLPM_DIR' environment variable",
            header_path.display()
        );
    }

    // tell cargo to re-run the build script if the header or environment variable changes
    println!("cargo:rerun-if-changed={}", header_path.display());
    println!("cargo:rerun-if-env-changed=TLMPX_DIR");

    // link against the pre-compiled library, only on windows
    if cfg!(target_os = "windows") {
        if let Some(parent) = include_dir.parent() {
            let lib_dir = parent.join(r"Lib_x64\msc");
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        }
        println!("cargo:rustc-link-lib=TLPMX_64");
    } else {
        println!(
            "cargo:warning=not targeting windows, skipping TLPMX_64 library linkage (only `cargo check` will work, not `cargo build`)"
        );
    }

    // generate the bindings
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg("-DBUILDING_DEBUG_EXE")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // restrict bindings to just the thorlabs and visa prefixes to keep the file size manageable
        .allowlist_function("TLPMX_.*")
        .allowlist_type("Vi.*")
        .allowlist_var("TLPM.*")
        .generate()
        .expect("unable to generate TLPMX bindings");

    // write the bindings to the cargo OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings to output directory");
}
