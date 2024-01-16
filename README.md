# unicode-math-class for WebAssembly

🦀 

## Development

This project is relatively tiny. **There's only three exports!**

In addition to the usual Rust tooling you'll need these tools installed. If
you're lazy you can use `make setup` to install all of them globally.

- Rust `wasm32-unknown-unknown` target
- [`cargo component`](https://github.com/bytecodealliance/cargo-component)

For testing you'll probably want these:

- [Node.js](https://nodejs.org/en)
- [jco](https://github.com/bytecodealliance/jco)
- [Deno](https://deno.com/)
- [Bun](https://bun.sh/)
- [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)

To get started, run `make build` to generate the `.wasm` component file. Then
use `make test` to run the end-to-end tests that import and use the `.wasm` file
to make sure it works. So far only JavaScript and Rust bindings are tested.

To create new releases you should **not** create new GitHub releases. Instead
use the `make publish` command **or** run [the `make publish` workflow] to
upload the artifact `.wasm` file and create a new release. Use `GHFLAGS=--draft`
or the `draft` input to create a draft release instead.

<!-- prettier-ignore-start -->
[the `make publish` workflow]: https://github.com/jcbhmr/unicode-math-class.wasm/actions/workflows/make-publish.yml
<!-- prettier-ignore-end -->
