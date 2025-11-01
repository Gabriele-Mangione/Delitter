# file: models.py
from __future__ import annotations

from typing import Dict, List, Literal, Optional

from pydantic import BaseModel, Field

# --- Type definitions ---
Category = Literal[
    "beverage_can", "plastic_bottle", "glass_bottle", "paper_cup", "snack_wrapper",
    "cigarette_butt", "straw", "cup_lid", "bag", "other"
]
Material = Literal["aluminium", "plastic", "glass", "paper", "cardboard", "metal", "other"]


class DetectedObject(BaseModel):
    """Represents a single detected litter object in an image."""
    category: Category
    material: Material
    brand: Optional[str] = None
    confidence: float = Field(ge=0.0, le=1.0)
    # Optional geometry (can be added later)
    # bbox_xywh: Optional[List[float]] = None  # [x,y,w,h] in pixels
    # mask_rle: Optional[str] = None


class LitterAnalysis(BaseModel):
    """Complete analysis result for a litter image."""
    objects: List[DetectedObject]
    counts: Dict[str, int]
    total_items: int
    weight_g_estimate: Optional[float] = None
    notes: Optional[str] = None
    processing_time_ms: Optional[float] = None  # Processing time in milliseconds
