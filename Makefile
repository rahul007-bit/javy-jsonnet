.PHONY: all
all: build-arakoo

add:
	@echo "Adding wasm32-wasi target"
	@rustup target add wasm32-wasi

clean-rm: clean-shims
	@rm -rf target/

clean:
	@cargo clean

clean-shims:
	@rm -rf dist/ node_modules/

build-arakoo: build-cors
	@echo "Building arakoo cli"
	@CARGO_PROFILE_RELEASE_LTO=off cargo build -p cli -r

build-cors: build-shims
	@echo "Building arakoo core"
	@cargo build -p arakoo-core --target=wasm32-wasi -r --features experimental_event_loop 

build-shims: shims-install
	@echo "Building shims"
	@cd crates/apis/src/http/shims && npm install

shims-install:
	@echo "Installing deps of shims"
	@cd crates/apis/src/http/shims && npm install

compile:
	./target/release/cli compile app.js

serve:
	./target/release/cli serve index.wasm

