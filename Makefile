watch :
	cargo watch -x fmt -s 'make build' -w src -w ../smithy/

build :
	mkdir -p dist
	cargo +nightly build --release --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/smithy_site_routing.wasm --out-dir ./dist

build_prod :
	rm -rf dist
	mkdir -p dist
	cargo +nightly build --release --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/smithy_site_routing.wasm --out-dir ./dist
	../binaryen/bin/wasm-opt -Oz -o dist/smithy_site_routing_bg.wasm dist/smithy_site_routing_bg.wasm
