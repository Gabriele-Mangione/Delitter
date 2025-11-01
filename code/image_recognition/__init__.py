# file: __init__.py
"""
Litter Analysis Package

AI-powered litter image analysis.
Provides both CLI and REST API interfaces.
"""

__version__ = "1.0.0"

from analyzer import analyze_image
from model_output_structure import Category, DetectedObject, LitterAnalysis, LitterDetection, Material

__all__ = [
    "analyze_image",
    "LitterAnalysis",
    "LitterDetection",
    "DetectedObject",
    "Category",
    "Material",
]
