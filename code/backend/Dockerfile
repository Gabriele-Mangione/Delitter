FROM rust:1.89-bullseye AS builder

ARG GIT_HASH=development

WORKDIR /usr/src/myapp

# Copy full source
COPY . .

# Install required tools and dependencies
RUN set -eux; \
	apt-get update && apt-get install -y --no-install-recommends ca-certificates git curl build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN set -eux; \
	cargo build --release

FROM debian:bullseye-slim

ARG GIT_HASH=development

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/myapp/target/release/myapp .

# Write version file
RUN echo "${GIT_HASH}" > version.txt

# Ensure runtime can validate TLS
RUN set -eux; \
	apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/* || true

# Expose the default port for the service
EXPOSE 8080

# Make binary executable and run it
RUN chmod +x ./myapp || true
CMD ["sh", "-c", "./myapp --port ${PORT:-8080}"]
