# tortoaster

## Prerequisites

```shell
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown

cargo install typeshare-cli

sudo npm install -g pnpm
pnpm install
```

## Build

```shell
typeshare -l typescript -o beegone_wasm/beegone.d.ts beegone

cargo build -p beegone --features wasm-bindgen --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir beegone_wasm target/wasm32-unknown-unknown/release/beegone.wasm

pnpm -C beegone_front_end run dev
```
