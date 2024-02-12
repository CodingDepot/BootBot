FROM rust:bullseye

WORKDIR /boot-bot
COPY . .

RUN apt update
RUN apt install gcc-arm-linux-gnueabihf -y

RUN rustup target add armv7-unknown-linux-gnueabihf
RUN cargo install --target armv7-unknown-linux-gnueabihf --path .
