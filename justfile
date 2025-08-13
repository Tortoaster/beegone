run:
    cd front-end && npm run dev
i:
    cd front-end && npm install
check:
    cd beegone && cargo build
    cd front-end && npm run check
lint:
    cd beegone && cargo clippy
    cd front-end && deno lint
fmt:
    cd beegone && cargo +nightly fmt
    cd front-end && deno fmt
build:
    cargo build --release --target wasm32-unknown-unknown --manifest-path beegone/Cargo.toml
    wasm-bindgen --browser --out-dir beegone-js/src beegone/target/wasm32-unknown-unknown/release/beegone.wasm
    bin/wasm-opt beegone-js/src/beegone_bg.wasm -o beegone-js/src/beegone_bg.wasm --all-features -O2
