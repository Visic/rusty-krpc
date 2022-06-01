use std::error::Error;
use bytes::BytesMut;
use prost::{Message, decode_length_delimiter};
use prost::encoding::encoded_len_varint;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod api { include!(concat!(env!("OUT_DIR"), "/krpc.api.rs")); }

#[derive(Debug)]
pub struct Client {
    buffer: BytesMut,
    stream: TcpStream,
    identifier: Vec<u8>,
}

impl Client {
    
}

pub async fn connect_to(addr: impl ToSocketAddrs, client_name: impl Into<String>) -> Result<Client, Box<dyn Error>> {
    //basic objects we'll need
    let mut stream = TcpStream::connect(addr).await?;
    let mut buffer = BytesMut::with_capacity(1024);

    //make the connection request, encode it into the buffer, and send it to the server
    let request = api::ConnectionRequest{
        client_name: client_name.into(),
        r#type: api::connection_request::Type::Rpc as i32,
        client_identifier: Default::default()
    };
    let request_len = request.encoded_len();
    buffer.reserve(request_len + encoded_len_varint(request_len as u64));
    request.encode_length_delimited(&mut buffer)?;
    stream.write_all(&buffer[..]).await?;

    //clear the buffer, read the server's response into it, and decode it as a connection response
    unsafe{ buffer.set_len(0); }
    let message_len = loop {
        buffer.reserve(buffer.len() + 1);
        unsafe{ buffer.set_len(buffer.len() + 1); }
        stream.read_exact(&mut buffer).await?;
        if let Ok(v) = decode_length_delimiter(&mut buffer) { break v; }
    };
    buffer.reserve(message_len);
    unsafe{ buffer.set_len(message_len); }
    stream.read_exact(&mut buffer).await?;
    let response = api::ConnectionResponse::decode(&buffer[..])?;

    //if the response is Ok, return our connected client, otherwise just return the error message
    (response.status == api::connection_response::Status::Ok as i32)
        .then_some(Client { buffer, stream, identifier: response.client_identifier })
        .ok_or_else(|| response.message.into())
}

#[cfg(test)]
mod tests {
    use crate::connect_to;

    #[tokio::test]
    async fn connection_test() {
        let result = connect_to("127.0.0.1:50000", "HomeSweetHome").await;
        println!("{:?}", result);
    }
}