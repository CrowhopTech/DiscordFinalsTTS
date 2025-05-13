# Build Stage
FROM rust:1.86-alpine AS builder
WORKDIR /usr/src/

RUN USER=root cargo new scavengerlabs
WORKDIR /usr/src/scavengerlabs
RUN apk update
RUN apk add openssl-dev musl-dev cmake make perl

COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Bundle Stage
FROM alpine:3.21.3
COPY --from=builder /usr/src/scavengerlabs/target/release/discord-finals-tts /discord-finals-tts
USER 1000
CMD ["/discord-finals-tts"]