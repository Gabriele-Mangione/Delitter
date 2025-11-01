FROM rust:1.89-bullseye as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/myapp/target/release/myapp .

# Expose a default port (can be overridden)
ENV APP_PORT=8080
EXPOSE ${APP_PORT}

# Run the binary, passing the env variable
RUN chmod 777 ./myapp
CMD ["sh", "-c", "./myapp --port $APP_PORT"]
