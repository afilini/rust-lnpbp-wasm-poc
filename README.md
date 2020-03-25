# rust-lnpbp-wasm-poc

1. Install Emscripten (normally done with emsdk)
2. Source the emsdk\_env.sh script to setup env variables
3. Add the `wasm32-unknown-emscripten` target to Rust
4. Build with `cargo build --target=wasm32-unknown-emscripten`
5. Start the webserver and open `http://localhost:8000`
