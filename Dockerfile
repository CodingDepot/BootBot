FROM rust:buster

WORKDIR /bootbot
COPY . .

RUN cargo install --path .

ENTRYPOINT ["boot_bot"]
