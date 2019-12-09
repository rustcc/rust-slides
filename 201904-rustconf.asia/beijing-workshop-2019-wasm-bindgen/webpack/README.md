## Example of using wasm-bindgen with [webpack](https://webpack.js.org/)

### Building

```
wasm-pack build --target bundler
```

Since `bundler` is the default target for `wasm-pack v0.8`,
we also can do

```
wasm-pack build
```

### Running

Install JavaScript dependencies

```
npm i
```

Start webpack dev server

```
npx webpack-dev-server
```

Then navigate to `localhost:port`
