use serde::Deserialize;
use std::{fs, io};
use thiserror::Error;

fn main() -> Result<(), ApiError> {
    let settings = load_settings("config.json");
    match settings {
        Ok(cfg) => {
            println!("Settings loaded: {:?}", cfg);
        }
        Err(e) => {
            eprintln!("Error loading settings: {}", e);
        }
    }

    let ping_result = send_ping("localhost:67000");
    match ping_result {
        Ok(_) => println!("Ping successful!"),
        Err(e) => eprintln!("Error sending ping: {}", e),
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum ApiError {
    // io:Error türündeki hataları otomatik olarak ApiError::Io varyantına dönüştürür.
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),

    // Ağ ile ilgili hataları temsil eder.
    #[error("Network Error: {0}")]
    Network(String),

    // JSON serileştirme/deserileştirme hatalarını temsil eder.
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
}

fn load_settings(path: &str) -> Result<Settings, ApiError> {
    let data = fs::read_to_string(path)?; // io::Error otomatik olarak ApiError::Io'ya dönüştürülür
    let settings: Settings = serde_json::from_str(&data)?; // serde_json::Error otomatik olarak ApiError::Json'a dönüştürülür
    Ok(settings)
}

fn send_ping(api_url: &str) -> Result<(), ApiError> {
    let response = std::net::TcpStream::connect(api_url);
    match response {
        Ok(_) => println!("Ping to {} successful!", api_url),
        Err(e) => return Err(ApiError::Network(e.to_string())),
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Settings {
    api_url: String,
    timeout: u64,
}
