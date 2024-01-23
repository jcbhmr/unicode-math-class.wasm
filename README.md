# unicode-math-class for WebAssembly

🦀 [unicode-math-class](https://crates.io/crates/unicode-math-class) Rust crate compiled to a [WebAssembly component](https://github.com/WebAssembly/component-model)

<table align=center><td>

```c
class('0') //=> "normal"
class('a') //=> "alphabetic"
class('(') //=> "opening"
class('😃') //=> null
```

</table>

<p align=center>
  <a href="https://jcbhmr.me/unicode-math-class.wasm/">Docs</a>
  | <a href="https://github.com/jcbhmr/unicode-math-class.wasm/releases/latest/download/unicode-math-class.wasm">Download</a>
</p>

🏷️ Uses unicode-math-class v0.1.0 \
🟪 Distributed as WebAssembly to run anywhere \
🤝 Composes well with other WebAssembly components \
0️⃣ Zero dependencies

👀 You might be looking for the premade [bindings for JavaScript](#TODO) or [bindings for Python](#TODO).

## Usage

![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)

You can download the [unicode-math-class.wasm](https://github.com/jcbhmr/unicode-math-class.wasm/releases/latest/download/unicode-math-class.wasm) WebAssembly component file from [this repository's GitHub releases page](https://github.com/jcbhmr/unicode-math-class.wasm/releases). To actually _use_ the WebAssembly component, you'll need to use a [WebAssembly component host runtime](https://github.com/jcbhmr/awesome-webassembly-runtimes#readme).

[![Download](https://img.shields.io/static/v1?style=for-the-badge&message=Download&color=0ABF53&logo=GitHub&logoColor=FFFFFF&label=)](https://github.com/jcbhmr/unicode-math-class.wasm/releases/latest/download/unicode-math-class.wasm)

How you use consume the [WIT-defined API](https://jcbhmr.me/unicode-math-class.wasm/) is determined by which [bindings generator](https://github.com/bytecodealliance/wit-bindgen) you use.

## Development

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)

Run `make setup` or install these additional tools:

- Rust `wasm32-unknown-unknown` target
- [`cargo component`](https://github.com/bytecodealliance/cargo-component)
- [`wit-bindgen` CLI](https://github.com/bytecodealliance/wit-bindgen#cli-installation)
- [Static Web Server](https://static-web-server.net/)
- [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)

You can run `make build` to build the `.wasm` file. Use `make build-docs` to build the docs. Run [the `make publish` workflow](https://github.com/jcbhmr/unicode-math-class.wasm/actions/workflows/make-publish.yml) or use `make publish` to create new releases.
