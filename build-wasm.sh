#!/bin/bash
# Build WASM module for rho-core

set -e

echo "Building rho-core WASM module..."

# Build the WASM
cd crates/rho-core
cargo build --target wasm32-unknown-unknown --features wasm --release
cd ../..

# Generate bindings
wasm-bindgen --target web --out-dir packages/rho-wasm crates/rho-core/target/wasm32-unknown-unknown/release/rho_core.wasm

echo "✓ WASM build complete!"
echo "  Files: packages/rho-wasm/"
ls -lh packages/rho-wasm/
