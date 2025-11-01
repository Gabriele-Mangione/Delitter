# file: litter_vision.py
from __future__ import annotations

import base64, json, sys
from pathlib import Path
from typing import List, Literal, Optional

from openai import OpenAI
from pydantic import BaseModel, ValidationError

from time import time

# --- API key aus Datei ---
API_KEY_PATH = Path("open_ai.key")
if not API_KEY_PATH.exists():
    raise SystemExit('Missing "open_ai.key". Put your OpenAI API key in that file.')
client = OpenAI(api_key=API_KEY_PATH.read_text(encoding="utf-8").strip())

# --- Minimal-Pydantic-Klassen für text_format ---  TODO: extend if needed
Category = Literal[
    "beverage_can", "plastic_bottle", "glass_bottle", "paper_cup",
    "snack_wrapper", "cigarette_butt", "cup_lid", "bag", "other",
]
Material = Literal["aluminium", "plastic", "glass", "paper", "cardboard", "metal", "other"]  # TODO: extend if needed

class LitterItem(BaseModel):
    category: Category
    material: Material
    brand: Optional[str] = None  # None, wenn unlesbar

class LitterList(BaseModel):
    litters: List[LitterItem]

SYSTEM_INSTRUCTIONS = (
    "You analyze cleanup photos and return ONLY JSON matching the schema. "
    "List each visible litter item (packaging/containers). "
    "If brand isn't readable, set it to null, same for material. Exclude tools/hands. No extra text."
)

def _data_url(p: Path) -> str:
    mime = "image/jpeg" if p.suffix.lower() in {".jpg", ".jpeg"} else "image/png"
    return f"data:{mime};base64,{base64.b64encode(p.read_bytes()).decode()}"

def analyze_image(image_path: str, model: str = "gpt-4o-2024-08-06") -> LitterList:
    p = Path(image_path)
    if not p.exists():
        raise FileNotFoundError(p)

    messages = [
        {"role": "system", "content": SYSTEM_INSTRUCTIONS},
        {"role": "user", "content": [
            {"type": "input_text", "text": "Analyze this photo and return litters only."},
            {"type": "input_image", "image_url": _data_url(p)},
        ]},
    ]

    # 1) Bevorzugt: Pydantic-Parsing via text_format
    resp = client.responses.parse(
        model=model,
        input=messages,
        text_format=LitterList,
        max_output_tokens=600,
    )
    parsed = getattr(resp, "output_parsed", None)
    if parsed is not None:
        return parsed  # bereits validiertes Pydantic-Objekt

    # 2) Sanfter Fallback: JSON-Text parsen + gegen Pydantic validieren
    raw = getattr(resp, "output_text", "") or ""
    try:
        data = json.loads(raw)
        return LitterList.model_validate(data)
    except Exception as e:
        raise SystemExit(
            "Model did not return a valid LitterList. "
            "Try a 4o snapshot or switch to response_format json_schema.\n"
            f"Raw model output (truncated): {raw[:800]}"
        ) from e

def main(argv: List[str]) -> None:
    if len(argv) < 2:
        print("Usage: python litter_vision.py <image_path> [model]")
        raise SystemExit(2)
    image_path = argv[1]
    model = argv[2] if len(argv) > 2 else "gpt-4o-2024-08-06"
    try:
        # timer
        start = time()
        result = analyze_image(image_path, model=model)
        print(f"Got result after {(time() - start):.2f}s\n", result)
    except ValidationError as ve:
        print("Validation failed:", ve, file=sys.stderr)
        raise

if __name__ == "__main__":
    main(sys.argv)
