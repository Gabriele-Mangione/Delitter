# file: api.py
"""REST API for litter image analysis."""
from __future__ import annotations

from pathlib import Path
from tempfile import NamedTemporaryFile
from typing import Annotated

from fastapi import FastAPI, File, HTTPException, UploadFile
from fastapi.middleware.cors import CORSMiddleware

from .analyzer import analyze_image
from .models import LitterDetection

# Initialize FastAPI app
app = FastAPI(
    title="Litter Analysis API",
    description="Analyze litter images to detect and classify waste items",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc",
)

# Enable CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Allow all origins - adjust for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/")
async def root():
    """Root endpoint - returns API info."""
    return {
        "name": "Litter Analysis API",
        "version": "1.0.0",
        "endpoints": {
            "analyze": "/v1/analyze",
            "docs": "/docs",
            "redoc": "/redoc",
        }
    }


@app.post("/v1/analyze", response_model=LitterDetection)
async def analyze_litter_image(
    file: Annotated[UploadFile, File(description="Image file (PNG or JPG) containing litter to analyze")]
) -> LitterDetection:
    """
    Analyze a litter image and return detected objects with metadata.

    Args:
        file: Uploaded image file (PNG or JPG)

    Returns:
        LitterDetection object containing analysis results, processing time, and model info

    Raises:
        HTTPException: If the file type is invalid or processing fails
    """
    # Validate file type
    if not file.content_type or not file.content_type.startswith("image/"):
        raise HTTPException(
            status_code=400,
            detail=f"Invalid file type: {file.content_type}. Must be an image (PNG or JPG)."
        )

    # Check file extension
    filename = file.filename or "image.jpg"
    suffix = Path(filename).suffix.lower()
    if suffix not in {".jpg", ".jpeg", ".png"}:
        raise HTTPException(
            status_code=400,
            detail=f"Invalid file extension: {suffix}. Must be .jpg, .jpeg, or .png"
        )

    try:
        # Read file content into memory
        contents = await file.read()

        # Create a temporary file to pass to analyze_image
        # (analyzer expects a file path, so we need to write to temp file)
        with NamedTemporaryFile(delete=False, suffix=suffix) as temp_file:
            temp_file.write(contents)
            temp_path = temp_file.name

        try:
            # Analyze the image (processing time is tracked inside)
            result = analyze_image(temp_path)
            return result
        finally:
            # Clean up temp file
            Path(temp_path).unlink(missing_ok=True)

    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="Image file not found")
    except Exception as e:
        raise HTTPException(
            status_code=500,
            detail=f"Error processing image: {str(e)}"
        )


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    return {"status": "healthy", "service": "litter-analysis-api"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
