# Quick Sequence Code

Quick Sequence Code (or simply QSC) is a simple application for generating sequences of QR codes from any file.

# References

[1] : https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm

# Build Instructions

## Prerequisites

- Rust >= ´1.91.0´
- Wasm-pack >= ´0.13.1´

## Build Command

´´´shell
wasm-pack build --target web --release
´´´

the pkg directory will be created in the root directory of the project containing the compiled WebAssembly module and JavaScript bindings.
