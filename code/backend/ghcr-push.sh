#!/usr/bin/env bash
set -euo pipefail

# ghcr-push.sh — build & push a local Docker image to GitHub Container Registry (GHCR)
# Requirements: gh (GitHub CLI), docker (or podman — see note), logged-in gh (`gh auth login`)
#
# Usage examples:
#   ./ghcr-push.sh                           # builds from ./Dockerfile and pushes :<git-sha> (fallback :local)
#   TAG=v1.0.0 ./ghcr-push.sh                # push with a custom tag (overrides git sha)
#   OWNER=my-org ./ghcr-push.sh              # push to an org instead of current repo owner
#   REPO=my-repo ./ghcr-push.sh              # override repo name (default: current repo)
#   CONTEXT=./app DOCKERFILE=./Dockerfile    \
#     ./ghcr-push.sh                         # custom build context & Dockerfile
#   BUILD_ARGS="--build-arg ENV=prod" ./ghcr-push.sh
#   PLATFORMS=linux/amd64,linux/arm64 ./ghcr-push.sh
#   ./ghcr-push.sh --emit-docker-auth-config # print JSON to use as Render's DOCKER_AUTH_CONFIG

# --- Config via env with sane defaults ---
REGISTRY="${REGISTRY:-ghcr.io}"

# Default TAG: short git SHA if available; otherwise "local". Can be overridden via env TAG=...
TAG="${TAG:-$(git rev-parse --short HEAD 2>/dev/null || true)}"
if [[ -z "${TAG}" ]]; then TAG="local"; fi

CONTEXT="${CONTEXT:-.}"
DOCKERFILE="${DOCKERFILE:-Dockerfile}"
BUILD_ARGS="${BUILD_ARGS:-}"                 # e.g. "--build-arg FOO=bar --build-arg BAZ=qux"
PLATFORMS="${PLATFORMS:-}"                   # e.g. "linux/amd64,linux/arm64" to use buildx
PUSH="${PUSH:-1}"                            # set to 0 to build only

EMIT_DOCKER_AUTH_CONFIG=0
if [[ "${1:-}" == "--emit-docker-auth-config" ]]; then
  EMIT_DOCKER_AUTH_CONFIG=1
fi

# --- Dependency checks ---
need() {
  command -v "$1" >/dev/null 2>&1 || { echo "Error: '$1' is required but not found in PATH." >&2; exit 1; }
}
need gh
need docker

# --- Resolve owner & repo from gh (unless overridden) ---
OWNER="${OWNER:-$(gh repo view --json owner -q .owner.login 2>/dev/null)}"
REPO="${REPO:-$(gh repo view --json name  -q .name         2>/dev/null)}"

if [[ -z "${OWNER}" || -z "${REPO}" ]]; then
  echo "Error: Could not determine OWNER/REPO. Run inside a GitHub repo or set OWNER/REPO env vars." >&2
  exit 1
fi

# --- Get a token from gh, ensure it has package scopes ---
TOKEN="$(gh auth token 2>/dev/null || true)"
if [[ -z "$TOKEN" ]]; then
  echo "Error: gh is not authenticated. Run: gh auth login" >&2
  exit 1
fi

# --- Docker login to GHCR ---
echo "$TOKEN" | docker login "$REGISTRY" -u "$OWNER" --password-stdin >/dev/null
echo "Logged in to $REGISTRY as $OWNER via gh token."

# --- Compose image name ---
IMAGE="${REGISTRY}/${OWNER}/${REPO}:${TAG}"
echo "Image: ${IMAGE}"

# --- Build (supports buildx for multi-arch if PLATFORMS is set) ---
if [[ -n "$PLATFORMS" ]]; then
  docker buildx version >/dev/null 2>&1 || { echo "Error: docker buildx not available."; exit 1; }
  echo "Building (multi-arch) with buildx for platforms: ${PLATFORMS}"
  docker buildx build \
    --platform "$PLATFORMS" \
    -f "$DOCKERFILE" \
    -t "$IMAGE" \
    $BUILD_ARGS \
    --push \
    "$CONTEXT"
  PUSH=0
else
  echo "Building image with docker build…"
  docker build -f "$DOCKERFILE" -t "$IMAGE" $BUILD_ARGS "$CONTEXT"
fi

# --- Push (if not already pushed by buildx) ---
if [[ "$PUSH" -eq 1 ]]; then
  echo "Pushing ${IMAGE}…"
  docker push "$IMAGE"
fi

echo "✅ Done. Pushed: ${IMAGE}"

# --- Optionally emit DOCKER_AUTH_CONFIG JSON for Render.com ---
if [[ "$EMIT_DOCKER_AUTH_CONFIG" -eq 1 ]]; then
  AUTH="$(printf '%s:%s' "$OWNER" "$TOKEN" | base64 | tr -d '\r\n')"
  cat <<JSON

--- Copy the JSON below as your Render secret 'DOCKER_AUTH_CONFIG' ---
{
  "auths": {
    "${REGISTRY}": {
      "auth": "${AUTH}"
    }
  }
}
--- End JSON ---
JSON
fi
