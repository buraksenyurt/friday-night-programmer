use crate::constants::*;
use crate::utility::*;
use log::*;
use std::io::ErrorKind;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Server {}

impl Server {
    pub async fn handle(socket: &mut TcpStream) -> tokio::io::Result<()> {
        send_response(socket, Command::ServerReady).await?;

        let mut buffer = [0; BUFFER_SIZE];
        let mut email_body = String::new();
        let mut data_mode = false;

        loop {
            match socket.read(&mut buffer).await {
                Ok(NO_BUFF_OR_EOF) => {
                    info!("Connection closed by client");
                    break;
                }
                Ok(n) => {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    info!("Received: {}", request);

                    if data_mode {
                        email_body.push_str(&request);

                        if email_body.ends_with(ENDS_WITH) {
                            let clean_body = email_body.trim_end_matches(ENDS_WITH);
                            if let Err(e) = save_email(clean_body) {
                                error!("Failed to save email: {}", e);
                            }
                            email_body.clear();

                            send_response(socket, Command::MessageAccepted).await?;
                            data_mode = false;
                        }
                    } else {
                        match request.trim() {
                            r if r.starts_with(Command::Hello.as_str())
                                || r.starts_with(Command::Hello2.as_str()) =>
                            {
                                send_response(socket, Command::HelloResponse).await?;
                            }
                            r if r.starts_with(Command::From.as_str()) => {
                                send_response(socket, Command::Ok).await?;
                            }
                            r if r.starts_with(Command::To.as_str()) => {
                                if r.contains(INVALID_MAIL) {
                                    send_response(socket, Command::ServiceNotAvailable).await?;
                                    error!("{}", Command::ErrorSimulated.as_str());
                                } else {
                                    send_response(socket, Command::Ok).await?;
                                }
                            }
                            r if r.starts_with(Command::Data.as_str()) => {
                                send_response(socket, Command::EndDataReply).await?;
                                data_mode = true;
                            }
                            r if r.starts_with(Command::Quit.as_str()) => {
                                send_response(socket, Command::Bye).await?;
                                info!("Session closed");
                                break;
                            }
                            _ => {
                                send_response(socket, Command::Unrecognized).await?;
                                error!("Unrecognized command received: {}", request);
                            }
                        }
                    }
                }
                Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                    warn!("Connection reset by peer");
                    break;
                }
                Err(e) => {
                    error!("Error reading from socket: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
