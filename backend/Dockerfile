
# Build Stage: Use Rust with MUSL for static linking
FROM rust:latest as builder

WORKDIR /app

# Install required MUSL dependencies
RUN apt-get update && apt-get install -y musl-tools musl-dev

# Install MUSL target for Rust
RUN rustup target add x86_64-unknown-linux-musl

# Copy source files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build with MUSL target
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Runtime Stage: Use a minimal Alpine Linux (musl-based)
FROM alpine:latest

WORKDIR /app

# Install minimal dependencies
RUN apk add --no-cache ca-certificates

# Copy the statically linked Rust binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /app/backend

# Expose the port
EXPOSE 4545 

# Run the application
CMD ["/app/backend"]
