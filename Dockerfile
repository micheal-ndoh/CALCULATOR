FROM rust:alpine AS builder

WORKDIR /app

COPY . .

RUN cargo build

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/debug/Calculator /app/calculator

CMD ["/app/calculator"]

