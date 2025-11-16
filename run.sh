#!/usr/bin/env bash
echo "Building wasm into release mode..."

wasm-pack build ./qsc-generator \
    --target web \
    --out-dir ../pkg \
    --release

echo "Running server..."
cargo run --release
