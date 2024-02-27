#!/bin/bash

cd cesrinf-wasm

wasm-pack build --target web --no-typescript --no-pack

cd ..

cp -v cesrinf-wasm/pkg/cesrinf_wasm.js cesrinfo-web/static/
cp -v cesrinf-wasm/pkg/cesrinf_wasm_bg.wasm cesrinfo-web/static/
