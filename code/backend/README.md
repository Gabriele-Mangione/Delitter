# Backend (Delitter)

Prerequisites
- Rust toolchain (rustup + cargo)
- MongoDB accessible for development

Run locally

1. Create a `.env` file or set `MONGO_URI` in your shell. Example `.env`:

```
MONGO_URI=mongodb://localhost:27017
```

2. Start the server:

```
cargo run
```

The server listens on 0.0.0.0:8080.

Run with Docker

1. Build image:

```
docker build -t delitter-backend .
```

2. Run with MongoDB available (example using a docker network):

```
docker network create delitter-net
docker run -d --name dev-mongo --network delitter-net mongo:7
docker run --rm --network delitter-net -e MONGO_URI="mongodb://dev-mongo:27017" -p 8080:8080 delitter-backend

Building behind a corporate proxy

If you're behind an HTTP/HTTPS proxy you can pass a proxy value as a build arg so the image can access crates.io and other network resources during the build. The Dockerfile accepts only `HTTPS_PROXY` (this keeps the surface small) and will configure git and cargo to use that proxy during the image build when provided.

Example build (uses the values from your shell):

```bash
# The project uses only `HTTPS_PROXY` and a corporate CA. Build like this:
export CORP_CA_B64=$(base64 -w0 "$CA_CERTIFICATES")
docker build \
  --build-arg HTTPS_PROXY="$HTTPS_PROXY" \
  --build-arg CORP_CA_B64="$CORP_CA_B64" \
  -t delitter-backend .

If your corporate proxy performs TLS interception (many do), you'll also need to provide
the corporate CA so the build container trusts intercepted TLS certificates. Provide the
PEM-encoded CA as base64 via the `CORP_CA_B64` build-arg. Example (from a shell):

```bash
# base64-encode the corp CA (we read the path from `CA_CERTIFICATES`) and pass it
export CORP_CA_B64=$(base64 -w0 "$CA_CERTIFICATES")
docker build \
  --build-arg HTTPS_PROXY="$HTTPS_PROXY" \
  --build-arg CORP_CA_B64="$CORP_CA_B64" \
  -t delitter-backend .
```
```

At runtime you can forward `HTTPS_PROXY` (and `MONGO_URI`) directly to the container:

```bash
docker run --rm -e MONGO_URI="mongodb://dev-mongo:27017" \
  -e HTTPS_PROXY="$HTTPS_PROXY" \
  -p 8080:8080 delitter-backend
```

Or use an `--env-file` (keep it out of VCS). Your `.env` can contain `HTTPS_PROXY` and `CA_CERTIFICATES`:

```bash
docker run --rm --env-file .env -p 8080:8080 delitter-backend
```
```

Quick test

Try sign-up (replace username/password as needed):

```
curl -X POST http://localhost:8080/v1/public/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"devuser","password":"devpass"}'
```

Notes
- The app requires `MONGO_URI` and uses a hard-coded JWT secret (`"secret"`) in the current code.
