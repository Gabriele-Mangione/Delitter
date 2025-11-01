FROM rust:1.89-bullseye AS builder

WORKDIR /usr/src/myapp

# Install required tools and dependencies
RUN set -eux; \
	apt-get update && apt-get install -y --no-install-recommends ca-certificates git curl build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Set build optimization environment variables
ENV CARGO_INCREMENTAL=0 \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    CARGO_TERM_COLOR=always

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to build dependencies (cached layer)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --locked && \
    rm -rf src

# Copy full source
COPY . .

# Build the actual application (only rebuilds if source changed)
RUN set -eux; \
	cargo build --release --locked

FROM debian:bullseye-slim

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/myapp/target/release/myapp .

# Ensure runtime can validate TLS
RUN set -eux; \
	apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/* || true

# Expose the default port for the service
EXPOSE 8080

# Make binary executable and run it
RUN chmod +x ./myapp || true
CMD ["sh", "-c", "./myapp --port ${PORT:-8080}"]
