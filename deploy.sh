#!/bin/sh

cargo build --release --target=wasm32-unknown-unknown
cd static
find . ! -name 'index.html' -type f -exec rm -f {} +
cd ../frontend/
yarn deploy
cd ..
cp target/wasm32-unknown-unknown/release/duplex.wasm static
wasm-gc static/duplex.wasm
