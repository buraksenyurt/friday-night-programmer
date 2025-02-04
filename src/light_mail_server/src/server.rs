use crate::constants::*;
use crate::utility::save_email;
use log::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Server {}

impl Server {
    pub async fn handle(socket: &mut TcpStream) -> tokio::io::Result<()> {
        socket
            .write_all(Command::ServerReady.as_str().as_bytes())
            .await?;
        let mut buffer = [0; BUFFER_SIZE];
        let mut email_body = String::new();
        let mut data_mode = false;

        loop {
            let n = socket.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            let request = String::from_utf8_lossy(&buffer[..n]);
            info!("Received: {}", request);

            if data_mode {
                email_body.push_str(&request);

                if email_body.ends_with(ENDS_WITH) {
                    let clean_body = email_body.trim_end_matches(ENDS_WITH);
                    save_email(clean_body)?;
                    email_body.clear();

                    socket
                        .write_all(Command::MessageAccepted.as_str().as_bytes())
                        .await?;
                    data_mode = false;
                }
            } else {
                if request.starts_with(Command::Hello.as_str())
                    || request.starts_with(Command::Hello2.as_str())
                {
                    socket
                        .write_all(Command::HelloResponse.as_str().as_bytes())
                        .await?;
                } else if request.starts_with(Command::From.as_str()) {
                    socket.write_all(Command::Ok.as_str().as_bytes()).await?;
                } else if request.starts_with(Command::To.as_str()) {
                    if request.contains(INVALID_MAIL) {
                        socket
                            .write_all(Command::ServiceNotAvailable.as_str().as_bytes())
                            .await?;
                        error!("{}", Command::ErrorSimulated.as_str());
                    } else {
                        socket.write_all(Command::Ok.as_str().as_bytes()).await?;
                    }
                } else if request.starts_with(Command::Data.as_str()) {
                    socket
                        .write_all(Command::EndDataReply.as_str().as_bytes())
                        .await?;
                    data_mode = true; // Start capturing the email body
                } else if request.starts_with(Command::Quit.as_str()) {
                    socket.write_all(Command::Bye.as_str().as_bytes()).await?;
                    info!("Session closed");
                    break;
                } else {
                    socket
                        .write_all(Command::Unrecognized.as_str().as_bytes())
                        .await?;
                    error!("Unrecognized command received: {}", request);
                }
            }
        }

        Ok(())
    }
}