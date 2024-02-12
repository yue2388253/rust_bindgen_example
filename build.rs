extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib_path = PathBuf::from(env::current_dir().unwrap().join("build"));

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=hello");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("hello_from_c")
        .generate()
        .expect("unable to generate hello bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
