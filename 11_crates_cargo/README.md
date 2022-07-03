# Cargo and Crates

## Library
Library doesnt have `main.rs` instead they have `lib.rs`. They can be imported in any module by mentioning in cargo.toml -> dependencies.

## Cargo
Cargo is the official Rust package management too. Just like npm for node.js .
Features 
- Dependency management and integration with crates.io (the official Rust package registry)
- Awareness of unit tests
- Awareness of benchmarks

cargo.toml spec [here](https://doc.rust-lang.org/cargo/reference/manifest.html).

To tell cargo to compile or run this binary as opposed to the default or other binaries, we just pass cargo the --bin my_other_bin flag, where my_other_bin is the name of the binary we want to work with.

