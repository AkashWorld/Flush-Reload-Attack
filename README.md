# Flush-Reload-Attack
Implementation of Side Channel Cache Attack - Educational Purposes Only!  
This project calibrates itself for the machine it runs on, and then probes the target executable (hardcoded as GnuPG for now) for cache memory accesses. It is based on the research paper listed in the *acknowledgement* section.

## Prerequisites
[Rust Build Tools](https://www.rust-lang.org/en-US/install.html)  
Linux (Tested on Ubuntu 16.04 LTS)  
x86-64  
This project uses inline assembly code. This is only available with the Rust Nightly Compiler. You may install it using the following command after downloading/installing Rust from the link above.
```
rustup install nightly
```

### Build
To build the project, go to the /src directory, and run the following command:
```
make release
```
Then run the executable at the root project directory with the path to GnuPG as an argument:
```
./target/release/flush-reload-attack /bin/gpg-1.4.13
```

### Built With
[Rust](https://www.rust-lang.org/en-US/) - The programming language used

### Test Platform
Linux Kernel 4.15.0  
Intel i7 6600, Dual Core  

### Acknowledgement
[FLUSH+RELOAD: a High Resolution, Low Noise, L3 Cache Side-Channel Attack](https://eprint.iacr.org/2013/448.pdf)
