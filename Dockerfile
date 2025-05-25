FROM rust:1.83 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/vecstore-extractor /app/app
CMD ["./app"]
