# tortoaster

## Prerequisites

```shell
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown

sudo npm install -g pnpm
pnpm install
```

## Build

```shell
cargo build -p beegone --features wasm-bindgen --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir beegone_wasm target/wasm32-unknown-unknown/release/beegone.wasm

npm run -w beegone_front_end dev
```
