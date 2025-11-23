[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/Lucas-BRT/quick-sequence-code/blob/main/LICENSE

# Quick Sequence Code

Quick Sequence Code (or simply QSC) is a simple application for generating sequences of QR codes from any file.

# References

1. https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm
2. https://en.wikipedia.org/wiki/QR_code

# Build Instructions

## Prerequisites

- Rust >= 1.88.0
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
