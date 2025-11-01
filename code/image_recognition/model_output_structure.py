# file: models.py
from __future__ import annotations

from typing import Dict, List, Literal, Optional

from pydantic import BaseModel, Field

# --- Type definitions ---
Category = Literal[
    "can", "bottle", "cigarette_butt", "snooze_pouch", "vape", "bag", "cup", 
    "snack_wrapper", "poop bags", "shard", "film_tarp_wrap", "cup_lid", "straw", "chewing_gum", "other"]

Material = Literal["aluminium", "plastic", "glass", "paper", "cardboard", "metal", "ruber", "textile", "other"]


class DetectedObject(BaseModel):
    """Represents a single detected litter object in an image."""
    category: Category
    material: Material
    weight_g_estimate: Optional[float] = None
    brand: Optional[str] = None
    confidence: float = Field(ge=0.0, le=1.0)
    # bounding_box: Optional[Dict[str, float]] = None  # e.g., {"x": 0.1, "y": 0.2, "width": 0.3, "height": 0.4}


class LitterAnalysis(BaseModel):
    """Analysis result from AI model - contains detected objects and computed statistics."""
    objects: List[DetectedObject]
    counts: Optional[Dict[str, int]] = None
    total_items: Optional[int] = None
    # weight_g_estimate: Optional[float] = None
    notes: Optional[str] = None


class LitterDetection(BaseModel):
    """Complete detection result including analysis and metadata."""
    analysis: LitterAnalysis
    processing_time_ms: float
    model: str
