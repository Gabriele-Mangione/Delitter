/// Test client for Delitter API
/// Usage: cargo run --bin test_api -- <username> <password> [file_or_directory]

use std::env;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    jwt: String,
}

#[derive(Serialize)]
struct LitterData {
    lat: f64,
    lng: f64,
    file: Vec<u8>,
    r#type: String,
}

#[derive(Deserialize)]
struct LitterCreateResponse {
    id: String,
}

#[derive(Deserialize, Debug)]
struct LitterGetData {
    id: String,
    lat: f64,
    lng: f64,
    category: String,
    material: String,
    weight: f64,
    brand: String,
    date: String,
}

struct ApiClient {
    base_url: String,
    client: reqwest::Client,
    token: Option<String>,
}

impl ApiClient {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
            token: None,
        }
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔐 Authenticating as '{}'...", username);

        let url = format!("{}/public/auth/signin", self.base_url);
        let payload = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let response = self.client.post(&url).json(&payload).send().await?;

        if !response.status().is_success() {
            return Err(format!("Authentication failed: {}", response.status()).into());
        }

        let auth_response: AuthResponse = response.json().await?;
        self.token = Some(auth_response.jwt);

        println!("✅ Authentication successful!\n");
        Ok(())
    }

    async fn upload_litter(
        &self,
        file_path: &str,
        file_num: Option<usize>,
        total_files: Option<usize>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let token = self
            .token
            .as_ref()
            .ok_or("Not authenticated. Please login first.")?;

        // Read file
        let file_bytes = fs::read(file_path)?;
        let file_size_kb = file_bytes.len() as f64 / 1024.0;

        // Get file extension
        let file_type = Path::new(file_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("jpg")
            .to_lowercase();

        let file_type = if file_type == "jpeg" {
            "jpg".to_string()
        } else {
            file_type
        };

        // Random coordinates (Europe)
        // Using system time as a simple random source
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let rand1 = (now.as_nanos() % 10000) as f64 / 10000.0;
        let rand2 = ((now.as_nanos() / 10000) % 10000) as f64 / 10000.0;
        let lat = 45.0 + rand1 * 10.0;
        let lng = 5.0 + rand2 * 10.0;

        // Print progress
        let filename = Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(file_path);

        match (file_num, total_files) {
            (Some(num), Some(total)) => {
                println!(
                    "[{}/{}] 📤 Uploading {} ({:.1} KB) at ({:.4}, {:.4})...",
                    num, total, filename, file_size_kb, lat, lng
                );
            }
            _ => {
                println!(
                    "📤 Uploading {} ({:.1} KB) at ({:.4}, {:.4})...",
                    filename, file_size_kb, lat, lng
                );
            }
        }

        // Create request
        let url = format!("{}/protected/litter", self.base_url);
        let payload = LitterData {
            lat,
            lng,
            file: file_bytes,
            r#type: file_type,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Upload failed: {}", response.status()).into());
        }

        let create_response: LitterCreateResponse = response.json().await?;
        println!("✅ Upload successful! ID: {}\n", create_response.id);

        Ok(create_response.id)
    }

    async fn get_all_litter(&self) -> Result<Vec<LitterGetData>, Box<dyn std::error::Error>> {
        let token = self
            .token
            .as_ref()
            .ok_or("Not authenticated. Please login first.")?;

        println!("📥 Fetching all litter reports...");

        let url = format!("{}/protected/litter", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to fetch litter: {}", response.status()).into());
        }

        let litter_items: Vec<LitterGetData> = response.json().await?;
        println!("✅ Found {} litter reports", litter_items.len());

        Ok(litter_items)
    }
}

fn collect_image_files(dir_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut image_files = Vec::new();
    let extensions = ["jpg", "jpeg", "png", "gif", "bmp"];

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if extensions.contains(&ext.to_lowercase().as_str()) {
                    if let Some(path_str) = path.to_str() {
                        image_files.push(path_str.to_string());
                    }
                }
            }
        }
    }

    Ok(image_files)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Delitter API Test Client\n");
        eprintln!("Usage: {} <username> <password> [file_or_directory]", args[0]);
        eprintln!("\nExamples:");
        eprintln!(
            "  {} myuser mypass test.jpg          # Upload single file",
            args[0]
        );
        eprintln!(
            "  {} myuser mypass ./images/         # Bulk upload",
            args[0]
        );
        eprintln!(
            "  {} myuser mypass                   # Just list litter",
            args[0]
        );
        eprintln!("\nEnvironment variables (.env file is loaded if present):");
        eprintln!("  BACKEND_URL - Backend API base URL (default: http://localhost:8080/v1)");
        eprintln!("  IMAGE_RECOGNITION_URL - Image recognition service URL");
        std::process::exit(1);
    }

    let username = &args[1];
    let password = &args[2];
    let target = args.get(3).map(|s| s.as_str());

    let base_url =
        env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

    let mut client = ApiClient::new(base_url);

    // Login
    client.login(username, password).await?;

    // Upload files if provided
    let mut upload_count = 0;

    if let Some(target_path) = target {
        let path = Path::new(target_path);

        if path.is_file() {
            // Single file upload
            client.upload_litter(target_path, None, None).await?;
            upload_count += 1;
        } else if path.is_dir() {
            // Bulk upload
            println!("📦 Scanning directory for images...");
            let image_files = collect_image_files(target_path)?;
            let total = image_files.len();

            if total == 0 {
                println!("❌ No image files found in {}", target_path);
            } else {
                println!("✅ Found {} image files\n", total);

                for (i, file_path) in image_files.iter().enumerate() {
                    match client.upload_litter(file_path, Some(i + 1), Some(total)).await {
                        Ok(_) => upload_count += 1,
                        Err(e) => eprintln!("❌ Error uploading {}: {}", file_path, e),
                    }
                }
            }
        } else {
            return Err(format!("Not a valid file or directory: {}", target_path).into());
        }

        // Summary
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 Summary: {} successful uploads", upload_count);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    // List all litter
    let litter_items = client.get_all_litter().await?;

    if !litter_items.is_empty() {
        println!("\nRecent reports (last 10):");
        for (i, item) in litter_items.iter().rev().take(10).enumerate() {
            let id = if item.id.len() > 12 {
                format!("{}...", &item.id[..12])
            } else {
                item.id.clone()
            };
            println!(
                "  {}. ID: {:15} | Location: ({:7.4}, {:7.4}) | Category: {:10} | Date: {}",
                i + 1,
                id,
                item.lat,
                item.lng,
                item.category,
                &item.date[..19.min(item.date.len())]
            );
        }

        if litter_items.len() > 10 {
            println!("  ... and {} more (showing last 10)", litter_items.len() - 10);
        }
    }

    println!("\n✅ Test completed successfully!");

    Ok(())
}
