FROM debian:latest

ARG GIT_HASH=development

WORKDIR /usr/local/bin
COPY ./target/release/myapp .

# Write version file
RUN echo "${GIT_HASH}" > version.txt

# Expose the default port for the service
ENV PORT=8080

EXPOSE $PORT

# Make binary executable and run it
RUN chmod +x ./myapp || true
CMD ["sh", "-c", "./myapp"]









