CARGO_CRATE_NAME := unicode_math_class
WORLD := unicode-math-class

build:
	cargo component build \
		--target wasm32-unknown-unknown \
		$(if $(RELEASE),--release) \
		$(CARGOFLAGS)
	ln \
		"$$PWD/target/wasm32-unknown-unknown/$(if $(RELEASE),release,debug)/$(CARGO_CRATE_NAME).wasm" \
		"$$PWD/target/wasm32-unknown-unknown/$(if $(RELEASE),release,debug)/$(WORLD).wasm"

setup:
	rustup target add wasm32-unknown-unknown
	command -v cargo-binstall || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	command -v cargo-component || cargo binstall cargo-component -y
	if ! command -v node; then \
		if [ "$$OS" = "Windows_NT" ]; then \
			curl -sS https://webi.sh/node | sh; \
		else \
			curl.exe https://webi.ms/node | powershell; \
		fi \
	fi
	command -v jco || npm install -g @bytecodealliance/jco
	command -v deno || curl -fsSL https://deno.land/install.sh | sh
	command -v bun || curl -fsSL https://bun.sh/install | bash
	command -v wit-bindgen || cargo binstall --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli -y
	command -v static-web-server || cargo binstall static-web-server -y
	command -v wasm-tools || cargo binstall wasm-tools -y

test-js:
	cd tests/js; \
	[ -d node_modules ] || npm install; \
	npm run build; \
	npm test

test-rs:
	# https://github.com/bytecodealliance/wasmtime/issues/7784
	# cargo test -p tests-rs

test-py:
	cd tests/py; \
	[ -d .venv ] || $(MAKE) venv; \
	$(MAKE) fetch; \
	$(MAKE) build; \
	$(MAKE) test

test: test-js test-rs test-py

publish:
	version=$$(cargo pkgid | cut -d'#' -f2 | cut -d'@' -f2); \
	echo -n | gh release create \
		"v$$version" \
		--generate-notes \
		$(GHFLAGS) \
		target/wasm32-unknown-unknown/release/$(WORLD).wasm

build-docs:
	wit-bindgen markdown wit --out-dir _site
	find _site -type f -name '*.md' -delete
	find _site -type f -name '*.html' -exec sed -i.bak '1i<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">' {} \;
	find _site -type f -name '*.bak' -delete
	echo '<meta http-equiv="refresh" content="0;url=$(WORLD).html">' > _site/index.html
	echo '<script>location.replace("$(WORLD).html");</script>' >> _site/index.html

docs-preview:
	static-web-server --root _site --port 8000 --cache-control-headers=false
