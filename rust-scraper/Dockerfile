# rust-scraper/Dockerfile
FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/rust-scraper .

EXPOSE 8080
CMD ["./rust-scraper"]