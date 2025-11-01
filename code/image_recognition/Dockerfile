# syntax=docker/dockerfile:1.4
# Use Python 3.11 slim image for smaller size
FROM python:3.11-slim

# Set working directory

WORKDIR /app

# Allow build-time proxy values to be passed in so the image can be built
# from behind corporate proxies. These are optional build args that will
# be propagated to ENV so pip and other tools can use them during the build.
ARG HTTP_PROXY
ARG http_proxy
ARG HTTPS_PROXY
ARG https_proxy
ARG NO_PROXY
ARG no_proxy

# Set the environment variables from the build args (if provided).
ENV HTTP_PROXY=${HTTP_PROXY}
ENV http_proxy=${http_proxy}
ENV HTTPS_PROXY=${HTTPS_PROXY}
ENV https_proxy=${https_proxy}
ENV NO_PROXY=${NO_PROXY}
ENV no_proxy=${no_proxy}

# Copy requirements first for better layer caching
# Copy requirements first for better layer caching
COPY requirements.txt .

# If a proxy is provided, create a pip config so pip uses it during install.
# This keeps pip from trying to reach the internet directly when behind a proxy.
RUN if [ -n "${HTTP_PROXY}${HTTPS_PROXY}${http_proxy}${https_proxy}" ]; then \
            mkdir -p /etc/pip.conf.d || true; \
            echo "[global]" > /etc/pip.conf; \
            # prefer https proxy if set, fall back to http proxy
            if [ -n "${HTTPS_PROXY}" ]; then echo "proxy = ${HTTPS_PROXY}" >> /etc/pip.conf; \
            elif [ -n "${https_proxy}" ]; then echo "proxy = ${https_proxy}" >> /etc/pip.conf; \
            elif [ -n "${HTTP_PROXY}" ]; then echo "proxy = ${HTTP_PROXY}" >> /etc/pip.conf; \
            elif [ -n "${http_proxy}" ]; then echo "proxy = ${http_proxy}" >> /etc/pip.conf; fi; \
        fi

# Install dependencies using a BuildKit cache mount for pip's cache directory.
# This speeds up subsequent builds by reusing downloaded wheels between runs.
RUN --mount=type=cache,target=/root/.cache/pip \
    pip install --no-cache-dir -r requirements.txt

# Copy the application code
COPY . .

# Expose the API port
ENV APP_PORT=${PORT:-8080}
EXPOSE ${APP_PORT}

# Set environment variables
ENV PYTHONUNBUFFERED=1

# Health check: use $PORT if set (Render sets a dynamic PORT). Fallback to 8000 for local runs.
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD python -c "import os,sys,urllib.request,sys as _s; port=os.environ.get('PORT','8000'); url='http://127.0.0.1:%s/health'%port; req=urllib.request.Request(url); urllib.request.urlopen(req, timeout=5); _s.exit(0)" || exit 1

# Run the API server: bind to $PORT (Render provides this env var). Fallback to 8000.
CMD ["sh", "-c", "uvicorn api:app --host 0.0.0.0 --port ${APP_PORT}"]
