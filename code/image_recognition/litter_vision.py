# file: litter_vision.py
from __future__ import annotations

import base64
import json
import sys
from pathlib import Path
from typing import Dict, List, Literal, Optional

from openai import OpenAI
from pydantic import BaseModel, Field, ValidationError

# --- read API key from local file: "open_ai.key" ---
API_KEY_PATH = Path("open_ai.key")
if not API_KEY_PATH.exists():
    raise SystemExit('Missing "open_ai.key". Put your OpenAI API key in that file.')
api_key = API_KEY_PATH.read_text(encoding="utf-8").strip()
client = OpenAI(api_key=api_key)

# --- Pydantic models for strict, structured output ---
Category = Literal[
    "beverage_can", "plastic_bottle", "glass_bottle", "paper_cup", "snack_wrapper",
    "cigarette_butt", "straw", "cup_lid", "bag", "other"
]
Material = Literal["aluminium", "plastic", "glass", "paper", "cardboard", "metal", "other"]

class DetectedObject(BaseModel):
    # id: str = Field(description="Stable ID within this image, like o1, o2…")
    category: Category
    material: Material
    brand: Optional[str] = None
    confidence: float = Field(ge=0.0, le=1.0)
    # Optional geometry (you can keep these None for now)
    # bbox_xywh: Optional[List[float]] = None  # [x,y,w,h] in pixels
    # mask_rle: Optional[str] = None

class LitterAnalysis(BaseModel):
    objects: List[DetectedObject]
    counts: Dict[str, int]
    total_items: int
    weight_g_estimate: Optional[float] = None
    notes: Optional[str] = None

# simple class→avg weight table; tune as you learn
AVG_WEIGHT_G = {
    "beverage_can": 14.0,
    "plastic_bottle": 20.0,
    "glass_bottle": 240.0,
    "paper_cup": 9.0,
    "snack_wrapper": 2.0,
    "cigarette_butt": 0.2,
    "straw": 0.5,
    "cup_lid": 2.5,
    "bag": 5.0,
    "other": 5.0,
}

def image_to_data_url(p: Path) -> str:
    mime = "image/jpeg" if p.suffix.lower() in {".jpg", ".jpeg"} else "image/png"
    b64 = base64.b64encode(p.read_bytes()).decode("utf-8")
    return f"data:{mime};base64,{b64}"

SYSTEM_INSTRUCTIONS = """\
You analyze photos of litter from cleanup events and return a strict JSON object that matches the provided schema.
Rules:
- Identify each visible litter item (not tools/hands), focusing on packaging/containers.
- For cans, set category=beverage_can and material=aluminium.
- Infer brand from visible text/logos if reasonably clear; else omit.
- Be conservative: only include items you can see; do not hallucinate masks/boxes.
- Provide counts by "category" and "material", plus a brief note if anything is uncertain.
"""

def analyze_image(image_path: str, model: str = "gpt-5") -> LitterAnalysis:
    """
    Returns a parsed LitterAnalysis. Requires a vision-capable model.
    """
    p = Path(image_path)
    if not p.exists():
        raise FileNotFoundError(p)

    # Compose a single user message with text + image (data URL)
    content = [
        {"type": "input_text", "text": "Extract litter objects, per the system rules."},
        {"type": "input_image", "image_url": image_to_data_url(p)},
    ]

    # Use Responses API with Pydantic parsing
    # (responses.parse returns .output_parsed as your Pydantic model)
    resp = client.responses.parse(
        model=model,
        input=[
            {"role": "system", "content": SYSTEM_INSTRUCTIONS},
            {"role": "user", "content": content},
        ],
        text_format=LitterAnalysis,  # ask for structured output as this Pydantic model
        max_output_tokens=800,
    )

    result: LitterAnalysis = resp.output_parsed  # already validated

    # If weight is missing, compute a quick estimate here
    if result.weight_g_estimate is None:
        w = 0.0
        for obj in result.objects:
            w += AVG_WEIGHT_G.get(obj.category, AVG_WEIGHT_G["other"])
        result.weight_g_estimate = round(w, 1)

    # If counts/total missing, compute here as a safety net
    if not result.counts:
        counts: Dict[str, int] = {}
        for obj in result.objects:
            counts[obj.category] = counts.get(obj.category, 0) + 1
        result.counts = counts
    if not result.total_items:
        result.total_items = sum(result.counts.values())

    return result

def main(argv: List[str]) -> None:
    if len(argv) < 2:
        print("Usage: python litter_vision.py <image_path> [model]")
        raise SystemExit(2)
    image_path = argv[1]
    model = argv[2] if len(argv) > 2 else "gpt-5"
    try:
        analysis = analyze_image(image_path, model=model)
        print(analysis.model_dump_json(indent=2, ensure_ascii=False))
    except ValidationError as ve:
        # Fall back to raw text if structured parse fails
        print("Validation failed:", ve, file=sys.stderr)
        # If you want, call responses.create and parse JSON manually here.
        raise

if __name__ == "__main__":
    main(sys.argv)
