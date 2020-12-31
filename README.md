## Installation
- Install [rustup](https://rustup.rs/)
- `rustup install nightly`
- `rustup target add wasm32-unknown-unknown`
- `cargo install wasm-pack just`

## Tech
- This front-end application is built using yew.
    * `wasm-pack build --target web --out-name wasm --out-dir ./static`
    * `miniserve ./static --index index.html` or just `just serve`
    * Access it at 'http://localhost:8080/'.
- The yew app is built using web-sys.

