FROM rust:buster

WORKDIR /bootbot
COPY . .

RUN cargo install --path .

CMD ["boot_bot"]
