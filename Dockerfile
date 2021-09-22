FROM rust:1.55.0-buster as base
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN cargo build --target wasm32-unknown-unknown --release

FROM rust:1.55.0-buster as target
COPY --from=base /target/wasm32-unknown-unknown/release/hello_world.wasm ./
