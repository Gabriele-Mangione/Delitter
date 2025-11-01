FROM rust:1.89-bullseye AS builder

# Minimal proxy inputs: only HTTPS_PROXY (used for build-time network access) and
# CORP_CA_B64 (optional base64 PEM of corporate CA). This reduces env var surface.
ARG HTTPS_PROXY
ARG CORP_CA_B64

WORKDIR /usr/src/myapp

# Expose HTTPS proxy inside the builder only (we deliberately omit HTTP_*/NO_PROXY variants)
ENV HTTPS_PROXY=${HTTPS_PROXY}

# Copy full source (kept simple here; consider cargo-layer optimization if desired)
COPY . .

# Install ca-certificates and tools needed during build, and configure git/cargo to use
# the provided proxy environment variables. If a corporate CA is provided (base64) it
# will be installed so TLS against intercepted connections validates correctly.
RUN set -eux; \
	apt-get update && apt-get install -y --no-install-recommends ca-certificates git curl build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN set -eux; \
	if [ -n "${CORP_CA_B64}" ]; then \
		echo "Installing corporate CA cert from build-arg"; \
		mkdir -p /usr/local/share/ca-certificates; \
		echo "${CORP_CA_B64}" | base64 -d > /usr/local/share/ca-certificates/corp.crt; \
		update-ca-certificates; \
	else \
		echo "No corporate CA provided, skipping CA install"; \
	fi

RUN set -eux; \
	if [ -n "${HTTPS_PROXY}" ]; then \
		echo "Configuring git and cargo to use proxy: ${HTTPS_PROXY}"; \
		git config --global http.proxy "${HTTPS_PROXY}" || true; \
		git config --global https.proxy "${HTTPS_PROXY}" || true; \
		mkdir -p /root/.cargo; \
		echo "[http]" > /root/.cargo/config.toml; \
		echo "proxy = \"${HTTPS_PROXY}\"" >> /root/.cargo/config.toml; \
	else \
		echo "No HTTPS_PROXY provided, skipping git/cargo proxy config"; \
	fi

RUN set -eux; \
	cargo build --release

FROM debian:bullseye-slim

# Runtime accepts only HTTPS_PROXY and optional CORP_CA_B64
ARG HTTPS_PROXY
ARG CORP_CA_B64

ENV HTTPS_PROXY=${HTTPS_PROXY}

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/myapp/target/release/myapp .

# Ensure runtime can validate TLS if a corporate CA is provided
RUN set -eux; \
	apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/* || true; \
	if [ -n "${CORP_CA_B64}" ]; then \
		mkdir -p /usr/local/share/ca-certificates; \
		echo "${CORP_CA_B64}" | base64 -d > /usr/local/share/ca-certificates/corp.crt; \
		update-ca-certificates || true; \
	fi

# Expose a default port (can be overridden)
ENV APP_PORT=${PORT:-8080}
EXPOSE ${APP_PORT}

# Make binary executable and run it. Use a non-root friendly permission.
RUN chmod +x ./myapp || true
CMD ["sh", "-c", "./myapp --port $APP_PORT"]
