FROM rust AS builder

WORKDIR /usr/src/app

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN apt install libssl-dev

RUN cargo install trunk

ARG BACKEND_URL

ENV BACKEND_URL=$BACKEND_URL

RUN  trunk build --release

FROM lipanski/docker-static-website:latest

COPY --from=builder /usr/src/app/dist .