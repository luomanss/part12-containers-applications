FROM rust AS builder

WORKDIR /usr/src/app

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN apt install libssl-dev

RUN cargo install trunk

RUN trunk build --release

FROM lipanski/docker-static-website:latest

COPY --from=builder /usr/src/app/dist .