FROM rust:alpine AS builder

WORKDIR /app

COPY . .

RUN cargo build

FROM alpine:latest

WORKDIR /app

<<<<<<< HEAD
COPY --from=builder /app/target/debug/Calculator /app/calculator

CMD ["/app/calculator"]
=======
COPY --from=builder /app/target/debug/Calculator /app/Calculator

CMD ["/app/Calculator"]
>>>>>>> a77fcac (added arithmetic functions and remove history file)
