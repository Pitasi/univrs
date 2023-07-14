# syntax=docker/dockerfile:1.3-labs

FROM rust:1.71-slim AS build

RUN cargo new /app
COPY Cargo.toml /app

WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

COPY ./src /app/src
COPY ./migrations /app/migrations

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  touch /app/src/main.rs
  cargo build --release
EOF

FROM debian:bullseye-slim AS app
COPY --from=build /app/target/release/univrs /univrs
CMD ["/univrs"]
