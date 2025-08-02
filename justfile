run:
    cd front-end && npm run dev
i:
    cd front-end && npm install
build:
    typeshare -l typescript -o beegone-js/src/types.d.ts beegone
    cargo build --release --target wasm32-unknown-unknown --manifest-path beegone/Cargo.toml
    wasm-bindgen --browser --out-dir beegone-js/src beegone/target/wasm32-unknown-unknown/release/beegone.wasm
    bin/wasm-opt beegone-js/src/beegone_bg.wasm -o beegone-js/src/beegone_bg.wasm --all-features -O2
