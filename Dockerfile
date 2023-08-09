# syntax=docker/dockerfile:1.3-labs

FROM rust:1.71-slim AS build

RUN apt update && apt install -y \
  libpq-dev \
  libssl-dev \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

RUN cargo new /app
COPY Cargo.toml /app

WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

COPY ./src /app/src
COPY ./migrations /app/migrations
COPY ./static/ /app/static/

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  touch /app/src/main.rs
  cargo build --release
EOF

FROM debian:bullseye-slim AS app
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && apt-get clean

COPY --from=build /app/target/release/univrs /univrs
COPY static/ static/
COPY articles/ articles/
CMD ["/univrs"]
