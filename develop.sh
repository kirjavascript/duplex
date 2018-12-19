#!/bin/sh

trap 'kill %1; kill %2' SIGINT
cargo watch -x 'build --target=wasm32-unknown-unknown' -s 'cp target/wasm32-unknown-unknown/debug/duplex.wasm static' -w src &
python -m http.server --directory static &
cd frontend && yarn watch
