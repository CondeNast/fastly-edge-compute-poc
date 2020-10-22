# stef-edge-computer

I am trying out some Compute@Edge stuff on Fastly! https://developer.fastly.com/learning/compute/

## Build and deploy it locally

This will compile the Rust code to a WASM+WASI target which means it **cannot** run natively on your computer.

```
fastly compute build
fastly compute validate -p pkg/stef-edge-computer.tar.gz
fastly compute deploy
```

## Run the tests locally

This will compile the Rust code to an x86_64 target which means it **can** run natively on your computer.

```
cargo test --all
```

## How does this work?

When you install and compile Rust code you can specify which compilation target you want to use. By default, Rust compiles for [x86](https://en.wikipedia.org/wiki/X86) systems when you compile using  tools such as `cargo` or `rustc` which are part of the default Rust toolchain. Fastly, on the other hand, compiles for [WASM](https://en.wikipedia.org/wiki/WebAssembly) + [WASI](https://en.wikipedia.org/wiki/WebAssembly#WASI) systems by default when you run `fastly compute build`.

Fastly is not doing anything special or magic. You can check this by running `cargo build --target wasm32-wasi` which should produce a `.wasm` build output in the `target` directory, the equivalent of running `fastly compute build` which produces a `.wasm` output in the `bin` directory.
