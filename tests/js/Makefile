WORLD := unicode-math-class

build:
	jco transpile \
		../../target/wasm32-unknown-unknown/$(if $(RELEASE),release,debug)/$(WORLD).wasm \
		-o out \
		--instantiation async \
		$(if $(RELEASE),--minify --optimize,--tracing) \
		$(JCOFLAGS)
	echo '{"type":"module"}' > out/package.json
