FROM debian:10

WORKDIR /bootbot
COPY . .

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN cargo install --path .

ENTRYPOINT ["boot_bot"]
