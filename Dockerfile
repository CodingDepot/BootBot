FROM rust:bullseye

WORKDIR /bootbot
COPY . .

RUN cargo install --target armv7-unknown-linux-gnueabihf --path .

ENTRYPOINT ["boot_bot"]
