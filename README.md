Rust
====

Code
----
rust-analyzer
CodeLLDB

Documentation
-------------

https://doc.rust-lang.org/book/

Cross Compiling for Windows
---------------------------

https://stackoverflow.com/questions/31492799/cross-compile-a-rust-application-from-linux-to-windows

Add the target and dependencies
```
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu

sudo apt-get install mingw-w64
```
Build your crate
```
cargo build --target x86_64-pc-windows-gnu
```

dmesg analyser
--------------

[dmesg log](./resources/dmesg_sample.log)
