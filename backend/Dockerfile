# Build Stage: Use Rust with MUSL for static linking
FROM rust:slim as builder

WORKDIR /app

# Install required dependencies including properly configured OpenSSL
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    musl-tools \
    libssl-dev \
    redis \
    ca-certificates \
    iproute2 \
    build-essential && \
    rm -rf /var/lib/apt/lists/* 
RUN redis-server --daemonize yes
RUN ss -plnt
ARG LLM_KEY
ARG GEMINI_KEY
ARG DEV
ARG BACKEND_PORT
ARG REDIS_PORT
ARG REDIS_HOST

ENV LLM_KEY=${LLM_KEY}
ENV GEMINI_KEY=${GEMINI_KEY}
ENV DEV=${DEV}
ENV BACKEND_PORT=${BACKEND_PORT}
ENV REDIS_PORT=${REDIS_PORT}
ENV REDIS_HOST=${REDIS_HOST}

RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml Cargo.lock ./
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