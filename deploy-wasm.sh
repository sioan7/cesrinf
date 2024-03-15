#!/bin/bash

cd cesrinf-wasm

wasm-pack build --target web --no-typescript --no-pack

cd ..

cp -v cesrinf-wasm/pkg/cesrinf_wasm.js web/static/
cp -v cesrinf-wasm/pkg/cesrinf_wasm_bg.wasm web/static/

# rm -R web/cesrinf-web-docker/static
# rm -R web/cesrinf-web-spin/static
# rm -R web/cesrinf-web-wasmer/static

# cp -r web/static web/cesrinf-web/
# cp -r web/static web/cesrinf-web-spin/
# cp -r web/static web/cesrinf-web-wasmer/
