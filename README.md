# rust_bindgen_example

This repository is a simple example to use [bindgen](https://github.com/rust-lang/rust-bindgen).

# How to use this repo

**Simply run ```make run``` is enough.** This command would compile the c code and generate a static lib (*libhello.a*) in directory *build*, and then run ```cargo run``` automatically.

```make``` : only compile the static lib (*libhello.a*)

```make clean``` : clean all the stuff that the commands above generate.