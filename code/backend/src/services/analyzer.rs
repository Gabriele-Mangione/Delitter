use std::env;

use log::{debug, error, info};
use reqwest::Client;
use reqwest::multipart::{Form, Part};

use serde::{Deserialize, Serialize};

pub async fn analyze(bytes: Vec<u8>) -> Result<Vec<DetectedObject>, Box<dyn std::error::Error>> {
    info!("image vec size: {}", bytes.len());
    let part = Part::bytes(bytes) // replace with bytes
        .file_name("image.jpg")
        .mime_str("image/jpg")?;

    let form = Form::new().part("file", part);

    let client = Client::new();

    let uri = env::var("IMAGE_RECOGNITION_URL").expect("IMAGE_RECOGNITION_URL not set");

    let response = client
        .post(format!("{uri}/v1/analyze"))
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    debug!("{response}");

    let response: Response = match serde_json::from_str(&response){
        Ok(r) => r,
        Err(e) => {
            error!("Error while parsing image {}", e);
            return Err(Box::new(e));
        }
    };

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
    pub category: Option<String>,
    pub material: Option<String>,
    pub weight_g_estimate: f64,
    pub brand: Option<String>,
    pub confidence: f64,
}
