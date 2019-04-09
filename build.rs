extern crate bindgen;

use std::process::Command;

fn build_libembroidery() {
    Command::new("qmake")
        .current_dir("./libembroidery")
        .output()
        .expect("Failed to build libembroidery: qmake");
    Command::new("make")
        .current_dir("./libembroidery")
        .output()
        .expect("Failed to build libembroider: make");
}

fn main() {
    build_libembroidery();
    println!("Built libembroidery!");

    println!("cargo:rustc-link-search=./libembroidery/lib");
    println!("cargo:rustc-link-lib=embroidery");

    let bindings = bindgen::Builder::default()
        .header("libembroidery.h")
        .generate()
        .expect("Failed to build!");

    bindings
        .write_to_file("./src/bindings.rs")
        .expect("Failed to write bindings");
}
