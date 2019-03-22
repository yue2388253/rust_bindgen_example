include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


unsafe fn call_hello_from_c(){
    hello();
}


fn main() {
    println!("Hello from Rust.");
    unsafe {
        call_hello_from_c();
    }
}