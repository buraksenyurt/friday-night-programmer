use crate::constants::*;
use log::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub fn save_email(body: &str) -> std::io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("{}/email_{}.txt", EMAIL_DIR, timestamp);

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .append(false)
        .open(&filename)?;

    file.write_all(body.as_bytes())?;
    info!("Email saved to {}", filename);

    Ok(())
}

pub async fn send_response(socket: &mut TcpStream, command: Command) -> tokio::io::Result<()> {
    socket
        .write_all(command.as_str().as_bytes())
        .await
        .map_err(|e| {
            error!("Failed to send response '{}': {}", command.as_str(), e);
            e
        })
}
