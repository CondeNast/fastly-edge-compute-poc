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
