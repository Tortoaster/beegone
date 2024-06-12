# beegone

## Prerequisites

```shell
cargo install wasm-pack
rustup target add wasm32-unknown-unknown

cargo install typeshare-cli

npm install -g pnpm
pnpm install
```

## Build

Repeat these commands every time the `beegone` library changes:

```shell
typeshare -l typescript -o wasm/types.d.ts beegone

wasm-pack build --release --out-dir ../wasm --scope beegone beegone
```

## Run

```shell
pnpm -C front_end run dev
```
