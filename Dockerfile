
FROM rust:latest as build


WORKDIR /usr/src/rust-htmx

COPY . .

COPY .env.docker .env
RUN apt-get update && apt-get install libpq5 -y

RUN cargo install --path .


FROM gcr.io/distroless/cc-debian11
ARG ARCH=aarch64
# Application files

COPY --from=build /usr/local/cargo/bin/rust-htmx /usr/local/bin/rust-htmx

COPY --from=build /usr/src/rust-htmx/.env /.env


CMD ["rust-htmx"]