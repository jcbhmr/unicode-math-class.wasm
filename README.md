# unicode-math-class for WebAssembly

🦀 [unicode-math-class] Rust crate compiled to a [WebAssembly component]

<p align=center>

</p>

🏷️ Uses [unicode-math-class] v0.1.0 \
🟪 Distributed as WebAssembly to run anywhere \
🤝 Composes well with other WebAssembly components

**👀 You might be looking for the [JavaScript bindings] or the [Python
bindings].**

## Usage

![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)

You can download the [unicode-math-class.wasm] WebAssembly component file from
[this repository's GitHub releases page]. To actually _use_ the WebAssembly
component, you'll need to use a [WebAssembly component host runtime].

[![Download](https://img.shields.io/static/v1?style=for-the-badge&message=Download&color=24A47F&logo=GitHub&logoColor=FFFFFF&label=)]()

## Development

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)
![JavaScript](https://img.shields.io/static/v1?style=for-the-badge&message=JavaScript&color=222222&logo=JavaScript&logoColor=F7DF1E&label=)

Run `make setup` or install these additional tools:

- Rust `wasm32-unknown-unknown` target
- [`cargo component`](https://github.com/bytecodealliance/cargo-component)
- [Node.js](https://nodejs.org/en)
- [jco](https://github.com/bytecodealliance/jco)
- [Deno](https://deno.com/)
- [Bun](https://bun.sh/)
- [`wit-bindgen` CLI](https://github.com/bytecodealliance/wit-bindgen#cli-installation)
- [Static Web Server](https://static-web-server.net/)

You can run `make build` to build the `.wasm` file. Use `make test` to test
using it in JavaScript and Rust. Use `make build-docs` to build the docs. Run
[the `make publish` workflow] or use `make publish` to create new releases.

<!-- prettier-ignore-start -->
[the `make publish` workflow]: https://github.com/jcbhmr/unicode-math-class.wasm/actions/workflows/make-publish.yml
[unicode-math-class]: https://crates.io/crates/unicode-math-class

<!-- prettier-ignore-end -->
