FROM debian:latest

WORKDIR /usr/local/bin
COPY ./target/release/myapp .

# Expose the default port for the service
ENV PORT=8080

EXPOSE $PORT

# Make binary executable and run it
RUN chmod +x ./myapp || true
CMD ["sh", "-c", "./myapp"]









