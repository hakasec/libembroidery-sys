extern crate bindgen;

use std::process::Command;
use std::collections::HashSet;

// IgnoreMacros courtesy of stillinbeta:
// https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547
#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

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

    // ignore oddly definited enums in math.h
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    bindgen::Builder::default()
        .header("libembroidery.h")
            .rustfmt_bindings(false)
            .parse_callbacks(Box::new(ignored_macros))
        .generate()
            .expect("Failed to build!")
        .write_to_file("./src/bindings.rs")
            .expect("Failed to write bindings!");
}
