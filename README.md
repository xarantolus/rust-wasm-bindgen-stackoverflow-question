This repo contains code for a StackOverflow question.

### Overview
1. Install the Rust toolchain, especially Cargo
2. [Install `wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
3. Run the following command to build for the web, then run a web server:
```
wasm-pack build --target web && python3 -m http.server
```
4. Now open http://localhost:8000/ (or link the python server outputs) in your browser and open the browser console
5. See the TODOs in [`src/lib.rs`](src/lib.rs) (in `run_function`) as well as [`index.html`](index.html) for the code that needs to be fixed


### Some notes
* [Using serde](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html) does not work here, as doing that the methods on the passed object are no longer available on the JavaScript side
