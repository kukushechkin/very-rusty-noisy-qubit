use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for native libraries in the specified directory
    println!("cargo:rerun-if-changed=very-noisy-qubit-api/include/very_noisy_qubit.h");
    println!("cargo:rerun-if-changed=tests/test.c");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("very-noisy-qubit-api/include/very_noisy_qubit.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
