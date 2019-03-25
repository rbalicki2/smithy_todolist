watch :
	cargo watch -x fmt -s 'make build' -w src -w ../smithy/ -w Cargo.toml

build :
	mkdir -p dist
	cargo +nightly build --release --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/smithy_todolist.wasm --out-dir ./dist

build_prod :
	rm -rf dist
	mkdir -p dist
	cargo +nightly build --release --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/smithy_todolist.wasm --out-dir ./dist
	../binaryen/bin/wasm-opt -Oz -o dist/smithy_todolist_bg.wasm dist/smithy_todolist_bg.wasm
