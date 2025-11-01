# Litter Analysis API

AI-powered litter image analysis tool that detects and classifies waste items . Available as both a CLI tool and REST API.

## Local Development Setup

### Prerequisites

- Python 3.11 or higher
- OpenAI API key

### Installation

1. Create and activate a virtual environment:
```bash
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

3. Configure environment variables:
```bash
# Copy the example file
cp .env.example .env

# Edit .env and add your OpenAI API key
# OPENAI_API_KEY=your-actual-api-key-here
```

**Important:** Keep your `.env` file secure and never commit it to version control (it's already in `.gitignore`).

## Usage

### CLI Tool

Analyze an image from the command line:

```bash
# Using default model
python -m image_recognition.litter_vision path/to/image.jpg

# Specify a custom model
python -m image_recognition.litter_vision path/to/image.jpg <model-name>
```

The CLI shows processing time and outputs the analysis as JSON.


### REST API Server

Start the API server:

```bash
# From within the image_recognition directory
uvicorn api:app --reload

# Production mode
uvicorn api:app --host 0.0.0.0 --port 8000
```

The API will be available at `http://localhost:8000`

### API Endpoints

- **`GET /`** - API information
- **`POST /v1/analyze`** - Analyze a litter image
- **`GET /health`** - Health check endpoint
- **`GET /docs`** - Interactive Swagger UI documentation
- **`GET /redoc`** - Alternative ReDoc documentation
- **`GET /openapi.json`** - OpenAPI schema

### Interactive API Documentation

Once the server is running, access the Swagger UI at:
```
http://localhost:8000/docs
```

### Example API Request

Using curl:
```bash
curl -X POST "http://localhost:8000/v1/analyze" \
  -H "accept: application/json" \
  -H "Content-Type: multipart/form-data" \
  -F "file=@path/to/image.jpg"
```

## Docker

### Build the Container

```bash
# Build the image
docker build -t litter-analysis-api .

# Verify the image was created
docker images | grep litter-analysis-api
```

#### Building behind a corporate proxy

If you're building behind a corporate HTTP/HTTPS proxy, pass the proxy values as build args so the image can access PyPI during the build. Example:

```bash
# Example using environment variables already set in your shell
docker build \
  --build-arg HTTP_PROXY="$HTTP_PROXY" \
  --build-arg HTTPS_PROXY="$HTTPS_PROXY" \
  -t litter-analysis-api .

# Or specify them inline (replace with your proxy)
docker build \
  --build-arg HTTP_PROXY="http://proxy.corp:8080" \
  --build-arg HTTPS_PROXY="http://proxy.corp:8080" \
  -t litter-analysis-api .
```

At runtime you can either pass the OpenAI key and any proxy environment variables directly:

```bash
docker run -d \
  -p 8000:8000 \
  -e OPENAI_API_KEY="your-api-key-here" \
  -e HTTP_PROXY="$HTTP_PROXY" \
  -e HTTPS_PROXY="$HTTPS_PROXY" \
  --name litter-api \
  litter-analysis-api
```

Or use an --env-file with the variables listed (keep it out of VCS):

```bash
docker run -d --env-file .env -p 8000:8000 --name litter-api litter-analysis-api
```

### Run the Container

```bash
# Run with environment variable
docker run -d \
  -p 8000:8000 \
  -e OPENAI_API_KEY="your-api-key-here" \
  --name litter-api \
  litter-analysis-api

# Or use .env file
docker run -d \
  -p 8000:8000 \
  --env-file .env \
  --name litter-api \
  litter-analysis-api

# Check logs
docker logs litter-api

# Stop the container
docker stop litter-api
docker rm litter-api
```

The API will be accessible at `http://localhost:8000`

## Development

### Running Tests

```bash
# Add tests in a tests/ directory
pytest
```

### Code Formatting

```bash
# Format with black
black .

# Lint with ruff
ruff check .
```
