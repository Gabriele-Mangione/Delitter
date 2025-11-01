# file: analyzer.py
from __future__ import annotations

import base64
import os
import time
from pathlib import Path
from typing import Dict

import httpx
from dotenv import load_dotenv
from openai import OpenAI

from model_output_structure import LitterAnalysis, LitterDetection

# --- Load environment variables ---
load_dotenv()

# --- OpenAI client setup ---
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    raise SystemExit('Missing OPENAI_API_KEY environment variable. Please set it in your .env file.')

# Configure proxy if set in environment
http_client = None
https_proxy = os.getenv("HTTPS_PROXY") or os.getenv("https_proxy")

if https_proxy:
    # Create httpx client with proxy configuration (OpenAI API uses HTTPS)
    # Note: verify=False disables SSL verification - needed for some corporate proxies
    http_client = httpx.Client(proxy=https_proxy, verify=False)
    print(f"[INFO] Using proxy: {https_proxy}")
else:
    print("[INFO] No proxy configured")

client = OpenAI(api_key=api_key, http_client=http_client)

# --- Constants ---
# Simple class → avg weight (in g) table; tune as you learn
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

SYSTEM_INSTRUCTIONS = """\
You analyze cleanup photos and return ONLY JSON matching the schema. No extra text.
Rules:
- List each visible litter item (packaging/containers only, not tools/hands).
- For cans: set category=beverage_can and material=aluminium.
- Infer brand from visible text/logos if absolutely certain; otherwise set to null.
- Be conservative: only include items you can see; do not hallucinate.
- Provide counts by category, plus weight estimate and notes if uncertain.
"""


def image_to_data_url(p: Path) -> str:
    """Convert an image file to a data URL for API submission."""
    mime = "image/jpeg" if p.suffix.lower() in {".jpg", ".jpeg"} else "image/png"
    b64 = base64.b64encode(p.read_bytes()).decode("utf-8")
    return f"data:{mime};base64,{b64}"


def analyze_image(image_path: str, model: str = "gpt-4o-2024-08-06") -> LitterDetection:
    """
    Analyze a litter image and return structured detection results.

    Args:
        image_path: Path to the image file to analyze
        model: OpenAI model to use (default: gpt-4o-2024-08-06)

    Returns:
        LitterDetection object with analysis results and metadata

    Raises:
        FileNotFoundError: If the image file doesn't exist
    """
    start_time = time.time()

    p = Path(image_path)
    if not p.exists():
        raise FileNotFoundError(p)

    # Compose a single user message with text + image (data URL)
    content = [
        {"type": "input_text", "text": "Extract litter objects, per the system rules."},
        {"type": "input_image", "image_url": image_to_data_url(p)},
    ]

    # Use Responses API with Pydantic parsing
    try:
        resp = client.responses.parse(
            model=model,
            input=[
                {"role": "system", "content": SYSTEM_INSTRUCTIONS},
                {"role": "user", "content": content},
            ],
            text_format=LitterAnalysis,  # ask for structured output as this Pydantic model
            max_output_tokens=800,
        )
    except Exception as api_error:
        print(f"[ERROR] OpenAI API error: {type(api_error).__name__}: {str(api_error)}")
        raise

    analysis: LitterAnalysis = resp.output_parsed  # already validated

    # If weight estimate is missing, compute a quick estimate here
    for obj in analysis.objects:
        if obj.weight_g_estimate is None:
            obj.weight_g_estimate = AVG_WEIGHT_G.get(obj.category, AVG_WEIGHT_G["other"])

    # If counts/total missing, compute here as a safety net
    if not analysis.counts:
        counts: Dict[str, int] = {}
        for obj in analysis.objects:
            counts[obj.category] = counts.get(obj.category, 0) + 1
        analysis.counts = counts
    if not analysis.total_items:
        analysis.total_items = sum(analysis.counts.values())

    # Calculate processing time and wrap in detection result
    processing_time_ms = (time.time() - start_time) * 1000

    return LitterDetection(
        analysis=analysis,
        processing_time_ms=round(processing_time_ms, 2),
        model=model
    )
