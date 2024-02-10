FROM rust:alpine3.19

WORKDIR /bootbot
COPY . .

RUN cargo install --path .

CMD ["boot_bot"]