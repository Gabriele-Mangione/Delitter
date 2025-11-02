# syntax=docker/dockerfile:1.4
# ============================================================================
# Dockerfile for Delitter - Rust Backend with Image Recognition
#
# PREREQUISITE: Build Rust backend first with:
#   cd code/backend && cargo build --release && cd ../..
# ============================================================================

FROM debian:bookworm-slim

ARG GIT_HASH=development

WORKDIR /app

# Install system dependencies for Rust binary
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the pre-built Rust binary (must be built locally first)
COPY code/backend/target/release/myapp /app/myapp
RUN chmod +x /app/myapp

# Write version file
RUN echo "${GIT_HASH}" > /app/version.txt

# Default environment variables (can be overridden at runtime)
ENV PORT=8080

# Expose the backend port
EXPOSE ${PORT}

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:${PORT:-8080}/alive || exit 1

# Start the Rust backend
CMD ["/app/myapp"]
