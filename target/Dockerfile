FROM rust:latest

WORKDIR /usr/src/app

COPY . .

# Build the Rust application
RUN cargo build --release

# Set the command to run the executable
CMD ["./target/release/Calculator"]



