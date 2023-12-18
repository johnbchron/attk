
run:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-name wasm_output --out-dir target --target web target/wasm32-unknown-unknown/release/attk.wasm
	sfz -p 3000 &> /dev/null
