FROM rust AS builder

WORKDIR /usr/src/phonebook-backend

COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update
RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/phonebook-backend /usr/local/bin/phonebook-backend

CMD ["phonebook-backend"]