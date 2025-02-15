FROM rust

WORKDIR /usr/src/phonebook-backend

RUN cargo install bacon

CMD ["bacon", "webserver", "--headless"]

