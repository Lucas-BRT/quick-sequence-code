# Quick Sequence Code

Quick Sequence Code (or simply QSC) is a simple application for generating sequences of QR codes from any file.

# References

1. https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm

# Build Instructions

## Prerequisites

- Rust >= 1.91.0
- Wasm-pack >= 0.13.1

## Build Command

Run the following command to build the WebAssembly module and JavaScript bindings:

```shell
wasm-pack build ./qsc-generator \
    --target web \
    --out-dir ../pkg \
    --release
```

The pkg directory will be created in the root directory of the project containing the compiled WebAssembly module and JavaScript bindings.

Now you can run the web server to serve the application.

```shell
cargo run --release
```

Or you can simply run the script by typing:

```shell
chmod +x ./run.sh && ./run.sh
```
