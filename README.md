A simple shoot 'em ups style game using Rust's Bevy crate.

Live demo (in webassembly) available at:

* https://play.marcoinacio.com


## Build instructions (executable)

* If you don't have `Rust` installed, see `https://rustup.rs`.

* Deploy with `cargo run --release`

* Your executable will be in ./target/release


## Build instructions (Wasm)

* If you don't have `Rust` installed, see `https://rustup.rs`.

* Then run the following commands:

* rustup target install wasm32-unknown-unknown

* cargo install -f wasm-bindgen-cli

* cargo build --release --target wasm32-unknown-unknown

* wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy-shoot-em-up.wasm

* The necessary files will be in ./out
