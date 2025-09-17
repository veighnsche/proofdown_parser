# Proofdown WASM Demo (minimal)

This is a minimal demo showing how to build and load the `proofdown_wasm` crate in a browser.

Build steps

1) Install wasm target and (optionally) wasm-pack:
   - `rustup target add wasm32-unknown-unknown`
   - `cargo install wasm-pack` (optional)

2) Build the WASM:
   - `cargo build -p proofdown_wasm --no-default-features --features wasm --target wasm32-unknown-unknown`
   - Output will be under `target/wasm32-unknown-unknown/debug/` as `proofdown_wasm.wasm`.

3) Serve the demo locally (any static server):
   - `python3 -m http.server` (or `npx serve`)
   - Open `index.html` in your browser: http://localhost:8000/examples/wasm/index.html

Notes

- This demo uses the raw `wasm_bindgen` JS glue. For production apps, prefer `wasm-pack` or bundlers.
- The parser remains syntax-only; validator runs on the Rust side (native). A WASM validator can be added later.
