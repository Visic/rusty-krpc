use std::error::Error;
use prost::Message;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod api { include!(concat!(env!("OUT_DIR"), "/krpc.api.rs")); }

pub struct Client {
    buffer: Vec<u8>,
    stream: TcpStream,
    identifier: Vec<u8>,
}

impl Client {
    
}

//TODO:: This isn't quiet right, we need to send/receive the message length (and then remove read_to_eof)

pub async fn connect_to(addr: impl ToSocketAddrs, client_name: impl Into<String>) -> Result<Client, Box<dyn Error>> {
    //basic objects we'll need
    let i32_len = (0i32).encoded_len();
    let mut stream = TcpStream::connect(addr).await?;
    let mut buffer = Vec::with_capacity(1024);

    //make the connection request, encode it into the buffer, and send it to the server
    let request = api::ConnectionRequest{
        client_name: client_name.into(),
        r#type: api::connection_request::Type::Rpc as i32,
        client_identifier: Default::default()
    };
    buffer.reserve(i32_len);
    (request.encoded_len() as i32).encode(&mut buffer[0..i32_len])?;
    request.encode(&mut buffer[i32_len..request.encoded_len()])?;
    stream.write_all(&buffer[..]).await?;

    //clear the buffer, read the server's response into it, and decode it as a connection response
    buffer.clear();
    buffer.reserve(i32_len);
    stream.read_exact(&mut buffer[0..i32_len]).await?;
    let message_len = i32::decode(&buffer[..])? as usize;
    buffer.clear();
    buffer.reserve(message_len);
    stream.read_exact(&mut buffer[0..message_len]).await?;
    let response = api::ConnectionResponse::decode(&buffer[..])?;

    //if the response is Ok, return our connected client, otherwise just return the error message
    (response.status == api::connection_response::Status::Ok as i32)
        .then_some(Client { buffer, stream, identifier: response.client_identifier })
        .ok_or_else(|| response.message.into())
}