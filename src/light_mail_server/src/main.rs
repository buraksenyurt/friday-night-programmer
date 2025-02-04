use crate::constants::*;
use crate::server::Server;
use log::*;
use std::env;
use tokio::fs::create_dir_all;
use tokio::net::TcpListener;

mod constants;
mod server;
mod utility;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    env::set_var("RUST_LOG", "tokio=info,light_mail_server=info");
    env_logger::init();

    create_dir_all(EMAIL_DIR).await?;

    let listener = TcpListener::bind(("0.0.0.0", 2525)).await?;
    info!("Light SMTP Server running on 0.0.0.0:2525");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Connection received from {}", addr);

        tokio::spawn(async move {
            Server::handle(&mut socket)
                .await
                .expect("Failed to run server");
        });
    }
}
