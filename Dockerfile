FROM rust:bullseye@sha256:3838b21cf7e71497a3b2fdd0ae50c75c91a7f499a15b78a0218c56829f02da00

WORKDIR /bootbot
COPY . .

RUN apt update
RUN apt install gcc-arm-linux-gnueabihf -y

RUN rustup target add armv7-unknown-linux-gnueabihf
RUN cargo install --target armv7-unknown-linux-gnueabihf --path .

ENTRYPOINT ["boot_bot"]
