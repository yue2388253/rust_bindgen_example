include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;


unsafe fn call_hello_from_c(){
    hello_from_c();
}


fn main() {
    println!("lib_path is [{}]", env::current_dir().unwrap().join("build").display());
    println!("Hello from Rust.");
    unsafe {
        call_hello_from_c();
    }
}