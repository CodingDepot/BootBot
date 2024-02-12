FROM rust:bullseye

WORKDIR /bootbot
COPY . .

RUN apt update
RUN apt install gcc-arm-linux-gnueabi

RUN rustup target add armv7-unknown-linux-gnueabihf
RUN cargo install --target armv7-unknown-linux-gnueabihf --path .

ENTRYPOINT ["boot_bot"]
