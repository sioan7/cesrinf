#!/bin/bash

# Prerequisites:
# - Wasmer https://github.com/wasmerio/wasmer

wasmer wasix build --release
wasmer run ./target/wasm32-wasmer-wasi/release/cesrinf-web-wasmer.wasm --net --env PORT=3000
