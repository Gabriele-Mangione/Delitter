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
