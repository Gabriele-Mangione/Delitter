# syntax=docker/dockerfile:1.4
# ============================================================================
# Unified Dockerfile for Delitter
# Combines Rust backend + Python image recognition in a single container
#
# PREREQUISITE: Build Rust backend first with:
#   cd code/backend && cargo build --release && cd ../..
# ============================================================================

FROM python:3.11-slim

ARG GIT_HASH=development
ARG HTTP_PROXY
ARG http_proxy
ARG HTTPS_PROXY
ARG https_proxy
ARG NO_PROXY
ARG no_proxy

# Set proxy environment variables if provided
ENV HTTP_PROXY=${HTTP_PROXY}
ENV http_proxy=${http_proxy}
ENV HTTPS_PROXY=${HTTPS_PROXY}
ENV https_proxy=${https_proxy}
ENV NO_PROXY=${NO_PROXY}
ENV no_proxy=${no_proxy}

WORKDIR /app

# Install system dependencies for Rust binary and supervisord
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    supervisor \
    && rm -rf /var/lib/apt/lists/*

# ----------------------------------------------------------------------------
# Set up Python Image Recognition Service
# ----------------------------------------------------------------------------

# Copy Python requirements and install dependencies
COPY code/image_recognition/requirements.txt /app/image_recognition/
RUN --mount=type=cache,target=/root/.cache/pip \
    pip install --no-cache-dir -r /app/image_recognition/requirements.txt

# Copy Python application code
COPY code/image_recognition/ /app/image_recognition/

# ----------------------------------------------------------------------------
# Set up Rust Backend
# ----------------------------------------------------------------------------

# Copy the pre-built Rust binary (must be built locally first)
COPY code/backend/target/release/myapp /app/backend/myapp
RUN chmod +x /app/backend/myapp

# ----------------------------------------------------------------------------
# Configure Supervisord
# ----------------------------------------------------------------------------

# Copy supervisord configuration
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

# ----------------------------------------------------------------------------
# Environment Configuration
# ----------------------------------------------------------------------------

# Write version file
RUN echo "${GIT_HASH}" > /app/version.txt && \
    echo "${GIT_HASH}" > /app/image_recognition/version.txt && \
    echo "${GIT_HASH}" > /app/backend/version.txt

# Set Python environment
ENV PYTHONUNBUFFERED=1

# Default environment variables (can be overridden at runtime)
ENV PORT=8080
ENV IMAGE_RECOGNITION_PORT=8081
ENV IMAGE_RECOGNITION_URL=http://localhost:8081

# Expose only the main backend port
EXPOSE ${PORT}

# ----------------------------------------------------------------------------
# Health Check
# ----------------------------------------------------------------------------

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:${PORT:-8080}/alive', timeout=5)" || exit 1

# ----------------------------------------------------------------------------
# Startup
# ----------------------------------------------------------------------------

# Start supervisord which will manage both services
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
