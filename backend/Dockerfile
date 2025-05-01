# Build Stage: Use Rust with MUSL for static linking
FROM rust:slim as builder

WORKDIR /app

# Install required dependencies including properly configured OpenSSL
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    musl-tools \
    libssl-dev \
    ca-certificates \
    build-essential && \
    rm -rf /var/lib/apt/lists/*

# Set OpenSSL environment variables correctly
ENV OPENSSL_DIR=/usr
ENV PKG_CONFIG_ALLOW_CROSS=1

# Install MUSL target for Rust
RUN rustup target add x86_64-unknown-linux-musl

# Create dummy source files first to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {println!(\"Dummy implementation\");}" > src/main.rs

# Copy only the dependency manifests first
COPY Cargo.toml Cargo.lock ./

# Build dependencies to cache them (this will fail but cache the deps)
RUN cargo build --release --target=x86_64-unknown-linux-musl || true

# Now copy the real source code
COPY src ./src

# Build the application with MUSL target
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Runtime Stage: Use a minimal Alpine Linux
FROM alpine:latest

WORKDIR /app

# Install minimal runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Copy the statically linked binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /app/backend

# Set environment variables for runtime if needed
# ENV RUST_LOG=info

# Expose the port your application uses
EXPOSE 4545

# Run the application
CMD ["/app/backend"]