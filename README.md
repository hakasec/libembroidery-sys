# libembroidery-sys
Rust FFI bindings for [libembroidery]. This was made using [rust-bindgen].

## Building
[libembroidery] is included as a submodule, 
so all you need to do is clone this repo and run in the cloned directory:

```
cargo build
```

To test the build, run the following:

```
cargo test
```

[libembroidery]: https://github.com/Embroidermodder/libembroidery
[rust-bindgen]: https://github.com/rust-lang/rust-bindgen
