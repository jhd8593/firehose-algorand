# Build stage
FROM rust:1.82 AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler
RUN cargo build --release --bin main

# Runtime stage
FROM debian:bookworm-slim

# Set environment variables to ensure UTF-8
ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    LANGUAGE=en_US:en

# Install all required system libraries
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3=3.0.* \
        openssl=3.0.* \
        libssl-dev=3.0.*; \
    rm -rf /var/lib/apt/lists/*

# Create app directory and copy binary
WORKDIR /app
COPY --from=builder /app/target/release/main /app/firehose-algorand

# Ensure the binary is executable
RUN chmod +x /app/firehose-algorand

# Set the working directory for volumes
WORKDIR /app/output

# Set the entrypoint
ENTRYPOINT ["/app/firehose-algorand"]

# Default command (can be overridden)
CMD ["--help"]
