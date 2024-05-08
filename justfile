desktop:
	cargo run --package game

web:
	@# build the wasm file
	@# use "--profile wasm-release" for maybe reduced wasm size
	@# and `--release` for prod code
	cargo build --package game --target wasm32-unknown-unknown
	@# build the wasm-bindgen js file
	wasm-bindgen --out-name krok_game \
		--out-dir web/public \
		--target web target/wasm32-unknown-unknown/debug/game.wasm
	@# start a web server
	basic-http-server web/

web-release:
	cargo build --package game --target wasm32-unknown-unknown --release
	wasm-bindgen --out-name krok_game \
		--out-dir web/public \
		--target web target/wasm32-unknown-unknown/release/game.wasm
	basic-http-server web/
