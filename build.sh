#!/bin/sh

rm -rf ./out

## Build WASM 
cargo build --release --target wasm32-unknown-unknown --features webgl2

## Generate Web output
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "infinity_graph" \
    ./target/wasm32-unknown-unknown/release/bevy_infinity_graph.wasm

cp ./temp/index.html ./out/index.html
cp ./temp/index.css ./out/index.css
mkdir ./out/assets/
mkdir ./out/assets/shaders/
cp ./assets/shaders/infinity.wgsl ./out/assets/shaders/infinity.wgsl

## Optimize build
cd out && wasm-opt -O -ol 100 -s 100 -o infinity_graph.wasm infinity_graph_bg.wasm

