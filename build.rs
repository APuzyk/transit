use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rgb_matrix_dir = PathBuf::from(&project_dir).join("rpi-rgb-led-matrix");

    // Run make in the rpi-rgb-led-matrix directory
    let status = Command::new("make")
        .current_dir(&rgb_matrix_dir)
        .args(&["clean", "all"])
        .status()
        .expect("Failed to execute make");

    if !status.success() {
        panic!("Make command failed");
    }

    // Tell Cargo where to find the compiled library
    println!("cargo:rustc-link-search={}", rgb_matrix_dir.join("lib").display());
    println!("cargo:rustc-link-lib=rgbmatrix");

    // Generate Rust bindings
    let bindings = bindgen::Builder::default()
        .header(rgb_matrix_dir.join("include/led-matrix-c.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Rerun the build script if the Makefile or C header changes
    println!("cargo:rerun-if-changed={}", rgb_matrix_dir.join("Makefile").display());
    println!("cargo:rerun-if-changed={}", rgb_matrix_dir.join("include/led-matrix-c.h").display());
}