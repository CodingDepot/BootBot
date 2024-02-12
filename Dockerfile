FROM rust:bullseye

WORKDIR /bootbot
COPY . .

RUN rustup target add armv7-unknown-linux-gnueabihf
RUN cargo install --target armv7-unknown-linux-gnueabihf --path .

ENTRYPOINT ["boot_bot"]
