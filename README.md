# CondeNast Fastly Compute@Edge Proof of Concept

An experiment in developing, building, testing, deploying and automation for [Fastly Compute@Edge](https://www.fastly.com/products/edge-compute/serverless) services.

The live version of this service is available at
https://condenast-poc.edgecompute.app/

## What is this?

Fastly Compute@Edge uses the [Lucet](https://github.com/bytecodealliance/lucet) runtime to let us run our own code in each of Fastly's points of presence around the world. This means that code can run much closer to our consumers and deliver faster responses. To do this we need to write code in a language that can be compiled to [WebAssembly](https://webassembly.org/), like [Rust](https://www.rust-lang.org/). This project is an experiment to understand the process and practices required to develop, build, test, and deploy code in this environment.

## How does this work?

When you install the Rust toolchain (or when compiling Rust code) you can specify which compilation target you want to use. By default, when you compile using  tools such as `cargo` or `rustc` which are part of the default Rust toolchain, Rust compiles for [x86](https://en.wikipedia.org/wiki/X86) systems. The Fastly CLI tool, on the other hand, compiles for [WASM](https://en.wikipedia.org/wiki/WebAssembly) + [WASI](https://en.wikipedia.org/wiki/WebAssembly#WASI) systems by default.

Fastly is not doing anything special or magic. You can check this by running `cargo build --target wasm32-wasi` which, assuming you have done the prerequisite steps for compiling WebAssembly on your computer, will produce a `.wasm` file in the `target` directory which is identical to the output produced from running `fastly compute build`.

## Developing

Follow the https://developer.fastly.com/learning/compute/ guide to start working on the code.

*Note:* If you're using Visual Studio Code and Docker you can use the provided [dev container](https://code.visualstudio.com/docs/remote/containers) to avoid installing dependencies locally. Use the `Remote-Containers: Open Workspace in Container` command.

### Test

This will compile our Rust code to an x86_64 target which means it can run natively on your computer, allowing us to test our code locally and in CI.

```
cargo test --all
```

### Build

This will compile all the Rust code (including the Fastly crates) to a WebAssembly target so it can run in the Fastly Compute@Edge environment.

```
fastly compute build
fastly compute validate -p pkg/fastly-edge-compute-poc.tar.gz
```

### Deploy

Deployment is managed by the continuous integration pipeline. All changes that are merged to the default branch are built, tested and then await a manual "hold" step before being deployed. This is done with the [Fastly CLI tool](https://developer.fastly.com/reference/cli/compute/deploy/).

```
fastly compute deploy --token ${FASTLY_API_TOKEN} --service-id ${FASTLY_SERVICE_ID}
```

See the [pipeline in CircleCI](https://circleci.deployment.cni.digital/gh/CondeNast/workflows/fastly-edge-compute-poc
) (requires authentication)
