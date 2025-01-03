# rust-iks

a rust implementation of NxN cube simulation, including an animated UI via wasm

## requirements

You have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.

## compilation

```sh
wasm-pack build . --target web --out-name web --out-dir ./pkg
```

## running

serve index.html from any web server and open it (after compiling)