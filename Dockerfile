FROM rust:bullseye

WORKDIR /boot-bot
COPY . .

RUN cargo install --path .

ENTRYPOINT ["boot_bot"]
