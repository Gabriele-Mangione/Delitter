use std::env;

use log::debug;
use reqwest::Client;
use reqwest::multipart::{Form, Part};

use serde::{Deserialize, Serialize};

pub async fn analyze(bytes: Vec<u8>) -> Result<Vec<DetectedObject>, Box<dyn std::error::Error>> {
    // let bytes = std::fs::read(std::path::Path::new("/home/andrea/Desktop/img.jpg"))?;
    let part = Part::bytes(bytes) // replace with bytes
        .file_name("image.jpg")
        .mime_str("image/jpg")?;

    let form = Form::new().part("file", part);

    let client = Client::new();

    let uri = env::var("ANALYZER_API").expect("ANALYZER_API not set");

    let response = client
        .post(format!("{uri}/v1/analyze"))
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    debug!("{response}");

    let response: Response = serde_json::from_str(&response)?;

    Ok(response.analysis.objects)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub analysis: Analysis,
    // pub processing_time_ms: u64,
    // pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub objects: Vec<DetectedObject>,
    // pub counts: HashMap<String, u64>,
    // pub total_items: u64,
    // pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedObject {
    pub category: String,
    pub material: String,
    pub weight_g_estimate: f64,
    pub brand: String,
    pub confidence: f64,
}
