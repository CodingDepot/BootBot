FROM rust:bullseye

WORKDIR /bootbot
COPY . .

RUN cargo install --path .

ENTRYPOINT ["boot_bot"]
