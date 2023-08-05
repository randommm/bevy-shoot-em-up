A simple shoot 'em up style game using Rust's Bevy crate with a mobile friendly version.

Live demo (in webassembly) available at:

* https://play.marcoinacio.com


## Build instructions (executable)

* If you don't have `Rust` installed, see `https://rustup.rs`

* Deploy with `cargo run --release`

* Your executable will be in ./target/release


## Build instructions (Wasm)

### Prerequirements

* If you don't have `Rust` installed, see `https://rustup.rs`

* Then run the following commands:

* `rustup target install wasm32-unknown-unknown`

* `cargo install -f wasm-bindgen-cli`

### Build the desktop version

* `cargo build --release --target wasm32-unknown-unknown`

* `wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-shoot-em-up.wasm`

* `cp -r ./assets ./out`

### Build the mobile version:

* `cargo build --release --target wasm32-unknown-unknown --features mobile`

* `wasm-bindgen --out-dir ./out/mobile --target web ./target/wasm32-unknown-unknown/release/bevy-shoot-em-up.wasm`

* `cp -r ./assets ./out/mobile`

### Results

* The necessary files will be available at ./out

### Optional extra optimazation

You can reduce the Wasm file size a little bit by running (must install `binaryen` first):

* `wasm-opt -Oz out/bevy-shoot-em-up_bg.wasm -o out/bevy-shoot-em-up_bg.wasm`

* `wasm-opt -Oz out/mobile/bevy-shoot-em-up_bg.wasm -o out/mobile/bevy-shoot-em-up_bg.wasm`
