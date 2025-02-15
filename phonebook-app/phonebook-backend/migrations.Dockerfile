FROM rust AS migration

WORKDIR /usr/src/app

COPY ./migrations /usr/src/app/migrations

RUN cargo install sqlx-cli --no-default-features --features postgres
