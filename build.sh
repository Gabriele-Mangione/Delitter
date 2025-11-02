#!/bin/bash
# Delitter Docker Build Script
set -e

echo "========================================"
echo "   Delitter Docker Build Script"
echo "========================================"
echo ""

# Get git hash for versioning
GIT_HASH=$(git rev-parse --short HEAD 2>/dev/null || echo "development")
echo "Git Hash: $GIT_HASH"
echo ""

# Step 1: Build Rust backend
echo "Step 1/3: Building Rust backend..."
cd code/backend
cargo build --release
cd ../..
echo "✓ Rust backend compiled"
echo ""

# Step 2: Build Docker image
echo "Step 2/3: Building Docker image..."
docker build \
  --build-arg GIT_HASH=$GIT_HASH \
  -t delitter:$GIT_HASH \
  -t delitter:latest \
  .
echo "✓ Docker image built: delitter:latest, delitter:$GIT_HASH"
echo ""

# Step 3: Show next steps
echo "Step 3/3: Build complete!"
echo ""
echo "========================================"
echo "   Next Steps"
echo "========================================"
echo ""
echo "To run the container:"
echo "  docker run -d -p 8080:8080 \\"
echo "    -e MONGO_URI=\"mongodb://localhost:27017\" \\"
echo "    -e OPENAI_API_KEY=\"your-key\" \\"
echo "    --name delitter delitter:latest"
echo ""
echo "To view logs:"
echo "  docker logs -f delitter"
echo ""
echo "To check service status:"
echo "  docker exec delitter supervisorctl status"
echo ""
echo "API Documentation:"
echo "  http://localhost:8080/docs/"
echo ""
echo "See DOCKER.md for detailed deployment instructions."
echo ""
