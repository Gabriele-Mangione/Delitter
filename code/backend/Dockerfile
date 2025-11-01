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
		# Try to decode the base64 payload; if decoding fails, print a helpful error
		if printf '%s' "${CORP_CA_B64}" | base64 -d > /usr/local/share/ca-certificates/corp.crt 2>/tmp/ca-decode.err; then \
			update-ca-certificates; \
		else \
			echo "ERROR: Failed to decode CORP_CA_B64. Base64 decode output:" >&2; \
			sed -n '1,200p' /tmp/ca-decode.err >&2 || true; \
			echo "Please ensure CORP_CA_B64 is valid base64 of a PEM file." >&2; \
			exit 1; \
		fi; \
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

# Expose the default port for the service. The runtime binary will read $PORT if provided
# when invoked; the image exposes 8080 as the default service port.
EXPOSE 8080

# Make binary executable and run it. Use a non-root friendly permission.
RUN chmod +x ./myapp || true
# Use shell expansion at runtime to prefer $PORT if set, otherwise default to 8080.
CMD ["sh", "-c", "./myapp --port ${PORT:-8080}"]
