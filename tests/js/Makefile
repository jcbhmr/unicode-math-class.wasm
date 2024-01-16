CARGO_CRATE_NAME := unicode_math_class_wasm

build:
	jco transpile \
		../../target/wasm32-unknown-unknown/$(if $(RELEASE),release,debug)/$(CARGO_CRATE_NAME).wasm \
		-o out \
		--instantiation async \
		$(if $(RELEASE),--minify --optimize,--tracing) \
		$(JCOFLAGS)
	echo '{"type":"module"}' > out/package.json
