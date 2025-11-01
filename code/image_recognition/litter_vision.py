# file: litter_vision.py
"""CLI tool for analyzing litter images using OpenAI vision models."""
from __future__ import annotations

import sys
from typing import List

from pydantic import ValidationError

# Support both direct execution and module execution
try:
    from .analyzer import analyze_image
except ImportError:
    from analyzer import analyze_image


def main(argv: List[str]) -> None:
    """CLI entry point for litter image analysis."""
    if len(argv) < 2:
        print("Usage: python litter_vision.py <image_path> [model]")
        raise SystemExit(2)

    image_path = argv[1]
    model = argv[2] if len(argv) > 2 else "gpt-4o-2024-08-06"

    try:
        # Analyze the image
        detection = analyze_image(image_path, model=model)

        # Display metadata
        print(f"Model: {detection.model}")
        print(f"Processing time: {detection.processing_time_ms:.2f}ms ({detection.processing_time_ms/1000:.2f}s)\n")

        # Display analysis results
        print(detection.analysis.model_dump_json(indent=2, ensure_ascii=False))
    except ValidationError as ve:
        # Fall back to raw text if structured parse fails
        print("Validation failed:", ve, file=sys.stderr)
        raise


if __name__ == "__main__":
    main(sys.argv)
