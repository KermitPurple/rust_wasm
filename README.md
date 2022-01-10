# wasm

## install wasm pack
```sh
cargo install wasm-pack
```

## build project
```sh
wasm-pack build --target web
```

## Use in html
[index.html](index.html)

## host locally
```sh
python3 -m http.server
```

### References
- [Mozilla Rust To Wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
- [Mozilla Web Asembly Concepts](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts)
- [Rust Wasm book](https://rustwasm.github.io/wasm-bindgen/introduction.html)
- [Wasmer Docs](https://docs.wasmer.io/integrations/js/wasi/browser/examples/handling-io)
