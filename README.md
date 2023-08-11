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

Repeat these commands every time the `beegone` library changes:

```shell
typeshare -l typescript -o beegone_wasm/beegone_types.d.ts beegone

cargo build -p beegone --features wasm-bindgen --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir beegone_wasm target/wasm32-unknown-unknown/release/beegone.wasm
```

## Run

```shell
pnpm -C beegone_front_end run dev
```
