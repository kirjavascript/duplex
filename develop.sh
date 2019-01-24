#!/bin/sh

# https://www.npmjs.com/package/local-web-server

trap 'kill %1; kill %2' SIGINT
cargo watch -x 'build --target=wasm32-unknown-unknown' -s 'cp target/wasm32-unknown-unknown/debug/duplex.wasm static' -w src &
# cargo watch -x 'build --release --target=wasm32-unknown-unknown' -s 'cp target/wasm32-unknown-unknown/release/duplex.wasm static && wasm-gc static/duplex.wasm' -w src &
ws --spa index.html --directory static &
cd frontend && yarn watch
