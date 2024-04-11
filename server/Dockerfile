FROM rust:1.77-bullseye as builder
WORKDIR /usr/src/cryptogame
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/cryptogame
COPY --from=builder /usr/local/cargo/bin/cryptogame ./
COPY config.json ./
CMD ["./cryptogame"]