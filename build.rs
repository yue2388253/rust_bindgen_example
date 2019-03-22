extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={}", out_path.display());
    println!("cargo:rustc-link-lib=hello");

    cc::Build::new()
        .file("src/hello.c")
        .include("src")
        .compile("hello");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("hello")
        .generate()
        .expect("unable to generate netlink-route bindings");
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
