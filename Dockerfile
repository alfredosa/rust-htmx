FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update && apt-get install -y musl-tools

RUN cargo build --release 
RUN /usr/src/app/target/release/rust-htmx