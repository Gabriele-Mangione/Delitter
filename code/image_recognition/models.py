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
    """Analysis result from AI model - contains detected objects and computed statistics."""
    objects: List[DetectedObject]
    counts: Optional[Dict[str, int]] = None
    total_items: Optional[int] = None
    weight_g_estimate: Optional[float] = None
    notes: Optional[str] = None


class LitterDetection(BaseModel):
    """Complete detection result including analysis and metadata."""
    analysis: LitterAnalysis
    processing_time_ms: float
    model: str
