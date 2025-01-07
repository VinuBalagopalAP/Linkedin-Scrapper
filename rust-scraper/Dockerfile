# rust-scraper/Dockerfile
FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Use Ubuntu instead of Debian
FROM ubuntu:22.04

# Install SSL and certificates
RUN apt-get update && \
    apt-get install -y openssl libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/rust-scraper .

EXPOSE 8080
CMD ["./rust-scraper"]