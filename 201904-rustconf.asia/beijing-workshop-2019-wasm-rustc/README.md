# The contents of the wasm workshop, part 1

Compiling to wasm using pure rustc

## Setting up

Add compilation target to the toolchain

```
rustup target add wasm32-unknown-unknown
```

## Building

```
rustc lib.rs --target wasm32-unknown-unknown --crate-type cdylib
```

## Running

Serve statics

```
http
````

Then navigate to localhost
