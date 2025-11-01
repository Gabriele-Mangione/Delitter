# file: models.py
from __future__ import annotations

from typing import Dict, List, Literal, Optional

from pydantic import BaseModel, Field

# --- Type definitions ---
Category = Literal[
    "Can", "Bottle", "Cigarette Butt", "Snooze Pouch", "Vape", "Bag", "Cup", 
    "Snack Wrapper", "Poop Bag", "Shard", "Film/Tarp/Wrap", "Cup Lid", "Straw", "Chewing Gum", "Other"]

Material = Literal["Aluminium", "Plastic", "Glass", "Paper", "Cardboard", "Metal", "Ruber", "Textile", "Other"]


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
