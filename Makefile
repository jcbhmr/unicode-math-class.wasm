CARGO_CRATE_NAME := unicode_math_class
COMPONENT_WORLD := unicode-math-class
CARGO_PKG_REPOSITORY := https://github.com/jcbhmr/unicode-math-class.wasm
TARGET := wasm32-unknown-unknown

build:
	cargo component build \
		--target $(TARGET) \
		$(if $(RELEASE),--release) \
		$(CARGOFLAGS)
	ln -f \
		"$$PWD/target/$(TARGET)/$(if $(RELEASE),release,debug)/$(CARGO_CRATE_NAME).wasm" \
		"$$PWD/target/$(TARGET)/$(if $(RELEASE),release,debug)/$(COMPONENT_WORLD).wasm"

setup:
	rustup target add $(TARGET)
	command -v cargo-binstall || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	command -v cargo-component || cargo binstall cargo-component -y
	command -v wit-bindgen || cargo binstall --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli -y
	command -v static-web-server || cargo binstall static-web-server -y
	command -v wasm-tools || cargo binstall wasm-tools -y

publish:
	echo -n | gh release create \
		"v$$(cargo pkgid | cut -d'#' -f2 | cut -d'@' -f2)" \
		--generate-notes \
		$(GHFLAGS) \
		target/$(TARGET)/release/$(COMPONENT_WORLD).wasm

build-docs:
	# TODO: Make a standalone WIT docgen tool
	wit-bindgen markdown wit --out-dir _site
	find _site -type f -name '*.md' -delete
	find _site -type f -name '*.html' -exec sed -i.bak '1i<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">' {} \;
	find _site -type f -name '*.bak' -delete
	find _site -type f -name '*.html' -exec sh -c 'echo "<p><a href=\"$(CARGO_PKG_REPOSITORY)\">Source on GitHub</a></p>" >> {}' \;
	echo '<meta http-equiv="refresh" content="0;url=$(COMPONENT_WORLD).html">' > _site/index.html
	echo '<script>location.replace("$(COMPONENT_WORLD).html");</script>' >> _site/index.html

docs-preview:
	static-web-server --root _site --port 8000 --cache-control-headers=false
