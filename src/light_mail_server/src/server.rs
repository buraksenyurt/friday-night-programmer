use tokio::net::TcpStream;

pub struct Server {}

impl Server {
    pub async fn handle(socket: &mut TcpStream) -> tokio::io::Result<()> {
        Ok(())
    }
}
