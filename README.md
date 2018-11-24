# Flush-Reload-Attack
Implementation of Side Channel Cache Attack - Educational Purposes Only!

## Prerequisites
[Rust Build Tools](https://www.rust-lang.org/en-US/install.html)  
Linux (Tested on Ubuntu 16.04 LTS)  
x86-64  
This project uses inline assembly code. This is only available with the Rust Nightly Compiler. You may install it using the following command after downloading the installing Rust.
```
rustup install nightly
```

### Build
To build the project, go to the root directory, and run the following command:
```
cargo +nightly build --release
```
Then run the executable at:
```
./target/release/flush-reload-attack
```

### Built With
[Rust](https://www.rust-lang.org/en-US/) - The programming language used

### Test Platform
Linux Kernel 4.15.0  
Intel i7 6600, Dual Core  

### Acknowledgement
[FLUSH+RELOAD: a High Resolution, Low Noise, L3 Cache Side-Channel Attack](https://eprint.iacr.org/2013/448.pdf)
