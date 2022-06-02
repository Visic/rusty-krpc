#![feature(explicit_generic_args_with_impl_trait)]
use std::error::Error;
use bytes::BytesMut;
use prost::{Message, decode_length_delimiter};
use prost::encoding::encoded_len_varint;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/krpc.api.rs"));

    impl Response {
        pub fn to_result(mut self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
            self.error.map(|v| Err(v))
                      .unwrap_or_else(||
                        self.results.drain(..)
                                    .map(|v| v.error.map(|v| Err(v))
                                                    .unwrap_or(Ok(v.value)))
                                    .collect::<Result<Vec<_>, _>>())
                      .map_err(|v| v.into())
        }
    }

    impl std::error::Error for Error {}
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Error")
                .field("name", &self.name)
                .field("service", &self.service)
                .field("description", &self.description)
                .field("stack_trace", &self.stack_trace)
                .finish()
        }
    }
}

#[derive(Debug)]
pub struct Client {
    buffer: BytesMut,
    stream: TcpStream,
    identifier: Vec<u8>,
}

impl Client {
    pub async fn connect_to(addr: impl ToSocketAddrs, client_name: impl Into<String>) -> Result<Client, Box<dyn Error>> {
        let mut client = Client {
            stream: TcpStream::connect(addr).await?,
            buffer: BytesMut::with_capacity(1024),
            identifier: Vec::default(),
        };
        let request = api::ConnectionRequest {
            client_name: client_name.into(),
            r#type: api::connection_request::Type::Rpc as i32,
            client_identifier: Default::default()
        };
        let response = client.transact::<api::ConnectionResponse>(request).await?;
        client.identifier = response.client_identifier;
        (response.status == api::connection_response::Status::Ok as i32).then_some(client).ok_or_else(|| response.message.into())
    }

    pub async fn get_status(&mut self) -> Result<api::Status, Box<dyn Error>> {
        let request = api::Request {
            calls: vec![
                api::ProcedureCall {
                    service: "KRPC".to_owned(),
                    procedure: "GetStatus".to_owned(),
                    ..Default::default()
                }
            ]
        };
        self.transact::<api::Response>(request).await?.to_result()?.pop().map_or_else(
            || Err("no api response".into()),
            |bytes| api::Status::decode(bytes.as_slice()).map_err(|e| e.into())
        )
    }

    pub async fn get_services(&mut self) -> Result<api::Services, Box<dyn Error>> {
        let request = api::Request {
            calls: vec![
                api::ProcedureCall {
                    service: "KRPC".to_owned(),
                    procedure: "GetServices".to_owned(),
                    ..Default::default()
                }
            ]
        };
        self.transact::<api::Response>(request).await?.to_result()?.pop().map_or_else(
            || Err("no api response".into()),
            |bytes| api::Services::decode(bytes.as_slice()).map_err(|e| e.into())
        )
    }

    async fn transact<RESP: Message + Default>(&mut self, request: impl Message) -> Result<RESP, Box<dyn Error>> {
        unsafe{ self.buffer.set_len(0); }
        let request_len = request.encoded_len();
        self.buffer.reserve(request_len + encoded_len_varint(request_len as u64));
        request.encode_length_delimited(&mut self.buffer)?;
        self.stream.write_all(&self.buffer[..]).await?;

        unsafe{ self.buffer.set_len(0); }
        let message_len = loop {
            let current_len = self.buffer.len();
            self.buffer.reserve(current_len + 1);
            unsafe{ self.buffer.set_len(current_len + 1); }
            self.stream.read_exact(&mut self.buffer[current_len..current_len + 1]).await?;
            if let Ok(v) = decode_length_delimiter(&mut self.buffer.clone()) { break v; }
        };
        self.buffer.reserve(message_len);
        unsafe{ self.buffer.set_len(message_len); }
        self.stream.read_exact(&mut self.buffer).await?;
        RESP::decode(&self.buffer[..]).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn connection_test() {
        let result = Client::connect_to("127.0.0.1:50000", "HomeSweetHome").await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn status_test() {
        let mut client = Client::connect_to("127.0.0.1:50000", "HomeSweetHome").await.unwrap();
        let result = client.get_status().await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn services_test() {
        let mut client = Client::connect_to("127.0.0.1:50000", "HomeSweetHome").await.unwrap();
        let result = client.get_services().await;
        println!("{:#?}", result);
    }
}