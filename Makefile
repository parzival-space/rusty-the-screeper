
# build tools
NPM = npm
WASM_PACK = wasm-pack


check-requirements:
	@command -v $(NPM) -v > /dev/null || (echo "npm is not installed. Please install it to proceed." && exit 1)
	@command -v $(WASM_PACK) --version > /dev/null || (echo "wasm-pack is not installed. Please install it to proceed." && exit 1)

# BUILD TARGETS

clean:
	rm -rf dist
	rm -rf pkg
	rm -rf node_modules
	rm -rf target

build-debug: check-requirements
	$(WASM_PACK) build --target web --debug --out-dir dist/wasm
	$(NPM) install -D
	$(NPM) run bundle

build-release: check-requirements
	$(WASM_PACK) build --target web --release --out-dir dist/wasm
	$(NPM) install -D
	$(NPM) run bundle

upload-debug: build-debug .screeps.yaml
	$(NPM) run upload

upload-release: build-release  .screeps.yaml
	$(NPM) run upload

# UTILS

get-memory: check-requirements
	$(NPM) run api memory $(FILTER)

get-all-segments: check-requirements
	$(NPM) run api segment all