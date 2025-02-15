FROM rust AS builder

WORKDIR /usr/src/app

RUN rustup target add wasm32-unknown-unknown

RUN apt install libssl-dev
RUN cargo install trunk

CMD trunk serve --address=$(hostname -I | awk '{print $1}')
