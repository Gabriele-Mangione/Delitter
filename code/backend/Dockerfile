FROM debian:latest

WORKDIR /usr/local/bin
COPY ./target/release/myapp .

# Expose the default port for the service
EXPOSE 8080

# Make binary executable and run it
RUN chmod +x ./myapp || true
CMD ["sh", "-c", "./myapp --port ${PORT:-8080}"]









