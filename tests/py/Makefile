PYTHON := python3.12
WORLD := unicode-math-class

.PHONY: build
build:
	. .venv/bin/activate; \
	python -m wasmtime.bindgen \
		../../target/wasm32-unknown-unknown/$(if $(RELEASE),release,debug)/$(WORLD).wasm \
		--out-dir build

fetch:
	. .venv/bin/activate; \
	python -m pip install -r requirements.txt

venv:
	$(PYTHON) -m venv .venv

shell:
	# add: python -m pip install <package>
	# remove: python -m pip uninstall <package>
	# freeze: python -m pip freeze > requirements.txt
	exec bash -c 'exec bash --rcfile <(echo ". ~/.bashrc; . .venv/bin/activate")'

test:
	. .venv/bin/activate; \
	python -m unittest
