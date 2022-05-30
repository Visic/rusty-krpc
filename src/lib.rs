use std::error::Error;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Client {
    stream: TcpStream,
}

pub async fn connect_to(addr: impl ToSocketAddrs) -> Result<Client, Box<dyn Error>> {
    let stream = TcpStream::connect(addr).await?;
    Ok(Client{stream})
}