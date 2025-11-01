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

# Install dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy the application code
COPY . .

# Expose the API port
EXPOSE 8000

# Set environment variables
ENV PYTHONUNBUFFERED=1

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD python -c "import requests; requests.get('http://localhost:8000/health')" || exit 1

# Run the API server
CMD ["uvicorn", "api:app", "--host", "0.0.0.0", "--port", "8000"]
