use std::collections::HashMap;
use std::env;
use std::time::Instant;

use async_openai::{
    Client,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, ChatCompletionRequestMessageContentPartImage,
        ChatCompletionRequestMessageContentPartText, ChatCompletionRequestUserMessageContentPart,
        CreateChatCompletionRequestArgs, ImageDetail, ImageUrl, ResponseFormat,
        ResponseFormatJsonSchema,
    },
};
use base64::{Engine as _, engine::general_purpose};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::litter::Entry;

const SYSTEM_INSTRUCTIONS: &str = r#"You analyze cleanup photos and return ONLY JSON matching the schema. No extra text.
Rules:
- List each visible litter item (packaging/containers only, not tools/hands).
- For cans: set category="Can" and material="Aluminium".
- Infer brand from visible text/logos if absolutely certain; otherwise set to null.
- Be conservative: only include items you can see; do not hallucinate.
- Provide counts by category, plus weight estimate and notes if uncertain."#;

const AVG_WEIGHT_G: &[(&str, &str, f64)] = &[
    ("Can", "Aluminium", 14.0),
    ("Bottle", "Plastic", 35.0),
    ("Bottle", "Glass", 240.0),
    ("Cup", "Paper", 9.0),
    ("Snack Wrapper", "", 2.0),
    ("Cigarette Butt", "", 0.2),
    ("Straw", "", 0.5),
    ("Cup Lid", "", 2.5),
    ("Bag", "", 5.0),
];

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedObject {
    pub category: String,
    pub material: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_g_estimate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LitterAnalysis {
    pub objects: Vec<DetectedObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub entries: Vec<Entry>,
    pub counts: Option<HashMap<String, i32>>,
    pub total_items: Option<i32>,
    pub notes: Option<String>,
    pub processing_time_ms: f64,
    pub model: String,
}

fn get_weight_estimate(category: &str, material: &str) -> f64 {
    AVG_WEIGHT_G
        .iter()
        .find(|(cat, mat, _)| *cat == category && (mat.is_empty() || *mat == material))
        .map(|(_, _, weight)| *weight)
        .unwrap_or(5.0)
}

fn image_bytes_to_data_url(bytes: &[u8], is_png: bool) -> String {
    let mime = if is_png { "image/png" } else { "image/jpeg" };
    let b64 = general_purpose::STANDARD.encode(bytes);
    format!("data:{};base64,{}", mime, b64)
}

fn create_json_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "objects": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "enum": [
                                "Can", "Bottle", "Cigarette Butt", "Snooze Pouch", "Vape",
                                "Bag", "Cup", "Snack Wrapper", "Poop Bag", "Shard",
                                "Film/Tarp/Wrap", "Cup Lid", "Straw", "Chewing Gum", "Other"
                            ]
                        },
                        "material": {
                            "type": "string",
                            "enum": [
                                "Aluminium", "Plastic", "Glass", "Paper", "Cardboard",
                                "Metal", "Ruber", "Textile", "Other"
                            ]
                        },
                        "weight_g_estimate": {
                            "type": "number",
                            "nullable": true
                        },
                        "brand": {
                            "type": "string",
                            "nullable": true
                        },
                        "confidence": {
                            "type": "number",
                            "minimum": 0.0,
                            "maximum": 1.0
                        }
                    },
                    "required": ["category", "material", "confidence"],
                    "additionalProperties": false
                }
            },
            "counts": {
                "type": "object",
                "additionalProperties": {
                    "type": "integer"
                },
                "nullable": true
            },
            "total_items": {
                "type": "integer",
                "nullable": true
            },
            "notes": {
                "type": "string",
                "nullable": true
            }
        },
        "required": ["objects"],
        "additionalProperties": false
    })
}

pub async fn analyze_image(image_bytes: Vec<u8>) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    info!("Starting image analysis, image size: {} bytes", image_bytes.len());

    // Verify API key is set (Client::new() will read it from environment)
    env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

    let client = Client::new();

    // Detect image format (simple heuristic: check for PNG magic bytes)
    let is_png = image_bytes.len() > 4 && &image_bytes[0..4] == b"\x89PNG";
    let data_url = image_bytes_to_data_url(&image_bytes, is_png);

    let model = "gpt-4o-2024-08-06";

    // Create JSON schema for structured output
    let schema = create_json_schema();

    // Build the request
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(SYSTEM_INSTRUCTIONS)
                    .build()?
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(vec![
                        ChatCompletionRequestUserMessageContentPart::Text(
                            ChatCompletionRequestMessageContentPartText {
                                text: "Extract litter objects, per the system rules.".to_string(),
                            }
                        ),
                        ChatCompletionRequestUserMessageContentPart::ImageUrl(
                            ChatCompletionRequestMessageContentPartImage {
                                image_url: ImageUrl {
                                    url: data_url,
                                    detail: Some(ImageDetail::Auto),
                                },
                            }
                        ),
                    ])
                    .build()?
            ),
        ])
        .response_format(ResponseFormat::JsonSchema {
            json_schema: ResponseFormatJsonSchema {
                name: "litter_analysis".to_string(),
                description: Some("Analysis of litter objects in an image".to_string()),
                schema: Some(schema),
                strict: Some(true),
            },
        })
        .max_tokens(800u32)
        .build()?;

    // Make the API call
    let response = client.chat().create(request).await.map_err(|e| {
        error!("OpenAI API error: {:?}", e);
        e
    })?;

    // Parse the response
    let content = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.as_ref())
        .ok_or("No content in response")?;

    let mut analysis: LitterAnalysis = serde_json::from_str(content).map_err(|e| {
        error!("Failed to parse OpenAI response: {:?}", e);
        error!("Response content: {}", content);
        e
    })?;

    info!("Parsed {} objects from OpenAI", analysis.objects.len());

    // Fill in missing weight estimates
    for obj in &mut analysis.objects {
        if obj.weight_g_estimate.is_none() {
            obj.weight_g_estimate = Some(get_weight_estimate(&obj.category, &obj.material));
        }
    }

    // Calculate counts if missing
    if analysis.counts.is_none() {
        let mut counts = HashMap::new();
        for obj in &analysis.objects {
            *counts.entry(obj.category.clone()).or_insert(0) += 1;
        }
        analysis.counts = Some(counts);
    }

    // Calculate total if missing
    if analysis.total_items.is_none() {
        analysis.total_items = Some(analysis.objects.len() as i32);
    }

    // Convert to Entry objects
    let entries: Vec<Entry> = analysis.objects.into_iter().map(|obj| Entry {
        category: obj.category,
        material: obj.material,
        weight_g_estimate: obj.weight_g_estimate,
        brand: obj.brand,
        confidence: obj.confidence,
    }).collect();

    let processing_time_ms = start_time.elapsed().as_secs_f64() * 1000.0;

    info!("Analysis completed in {:.2}ms", processing_time_ms);

    Ok(AnalysisResult {
        entries,
        counts: analysis.counts,
        total_items: analysis.total_items,
        notes: analysis.notes,
        processing_time_ms,
        model: model.to_string(),
    })
}
