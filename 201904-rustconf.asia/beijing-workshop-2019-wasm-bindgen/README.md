# The contents of the wasm workshop, part 2

Examples of using `wasm-bindgen`:
 - with [es6](es6) modules
 - with [node.js](nodejs)
 - with [webpack](webpack)

## Setting up

Add compilation target to the toolchain

```
rustup target add wasm32-unknown-unknown
```

Install `wasm-pack`

```
cargo install wasm-pack --version 0.8.1
```

Optionally, install static server

```
cargo install https
```

For `node.js` and `webpack` examples
you will need `node.js` binary installed.<br>
For detailed instructions visit https://nodejs.org/en/download/

### Useful links

- https://rustwasm.github.io/docs/wasm-bindgen/
- https://rustwasm.github.io/docs/wasm-pack/
- https://nodejs.org
- https://webpack.js.org/
