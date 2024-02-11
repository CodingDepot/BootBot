FROM debian:10

WORKDIR /bootbot
COPY . .

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
RUN cargo install --path .

ENTRYPOINT ["boot_bot"]
