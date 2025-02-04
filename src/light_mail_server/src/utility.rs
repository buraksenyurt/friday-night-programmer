use crate::constants::EMAIL_DIR;
use log::{error, info};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::create_dir_all;

pub fn save_email(body: &str) -> std::io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("{}/email_{}.txt", EMAIL_DIR, timestamp);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(&filename)?;

    file.write_all(body.as_bytes())?;
    info!("Email saved to {}", filename);

    Ok(())
}
