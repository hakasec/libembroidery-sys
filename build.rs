extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::process::Command;

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

#[derive(Clone)]
struct Paths {
    out_path: PathBuf,
    out_libpath: PathBuf,
    manifest_path: PathBuf,
    manifest_libpath: PathBuf,
}

fn config_paths() -> Paths {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let manifest_libpath = manifest_path.join("libembroidery");
    let out_libpath = out_path.join("libembroidery");

    Paths {
        out_path,
        out_libpath,
        manifest_path,
        manifest_libpath,
    }
}

fn build_libembroidery(p: Paths) {
    let manifest = p.manifest_path.to_str().unwrap();
    let manifest_lib = p.manifest_libpath.to_str().unwrap();
    let out_lib = p.out_libpath.to_str().unwrap();

    Command::new("cp")
        .args(&["-r", manifest_lib, out_lib])
        .current_dir(manifest)
        .output()
        .expect("Failed to copy libembroidery to OUT_DIR");
    Command::new("qmake")
        .current_dir(out_lib)
        .output()
        .expect("Failed to build libembroidery: qmake");
    Command::new("make")
        .current_dir(out_lib)
        .output()
        .expect("Failed to build libembroidery: make");
}

fn main() {
    let p = config_paths();

    build_libembroidery(p.clone());
    println!("Built libembroidery!");

    println!(
        "cargo:rustc-link-search={}",
        p.out_libpath.join("lib").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=embroidery");

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
        .write_to_file(p.out_path.join("bindings.rs"))
        .expect("Failed to write bindings!");
}
