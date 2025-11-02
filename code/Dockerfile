# syntax=docker/dockerfile:1.4
FROM docker:27-dind

# Install Docker Compose and other utilities
RUN apk add --no-cache docker-cli-compose bash git

WORKDIR /workspace

# Copy the entire repository into the container
COPY . .

RUN docker-compose build

# Default command: Run Docker Compose to start services
CMD ["sh", "-c", "docker-compose up --build"]
