CARGO_CRATE_NAME := unicode_math_class_wasm

build:
	cargo component build \
		--target wasm32-unknown-unknown \
		$(if $(RELEASE),--release) \
		$(CARGOFLAGS)

setup:
	rustup target add wasm32-unknown-unknown
	command -v cargo-binstall || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	command -v cargo-component || cargo binstall cargo-component -y
	command -v jco || npm install -g @bytecodealliance/jco
	command -v deno || curl -fsSL https://deno.land/install.sh | sh
	command -v bun || curl -fsSL https://bun.sh/install | bash
	command -v wasm-tools || cargo binstall wasm-tools -y
	command -v wasmtime || curl https://wasmtime.dev/install.sh -sSf | bash

test-js:
	(cd tests/js; npm install; npm run build; npm test)

test-rs:
	cargo test -p tests-rs

test: test-js test-rs

publish:
	
