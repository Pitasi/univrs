FROM rust:1.70-slim as builder

RUN USER=root cargo new --bin app
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build  --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

# Copy application binary from builder image
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin

EXPOSE 3000

# Run the application
CMD ["/usr/local/bin/app"]
