# Use an official Rust image as a build stage
FROM rust:latest as builder

WORKDIR /app

# Copy the source code and build it
COPY . .
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bullseye-slim
WORKDIR /app

# Install necessary runtime dependencies (if any)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Expose the port that your application will run on
EXPOSE 5353 

# Set the command to run the application
CMD ["/app/backend"]