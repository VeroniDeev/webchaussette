use crate::{
    handshake::{create_response, parse_request},
    utils::{build_response, generate_accept},
    websocket_types::ResponseStruct,
};
use std::{error::Error, sync::Arc};

use bytes::Bytes;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(url: &str) -> Self {
        let listener: TcpListener = TcpListener::bind(url).await.unwrap();
        Self { listener }
    }

    async fn handshake(&self, mut socket: TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut response: String = String::new();
        match socket.read(&mut buffer).await {
            Ok(n) => {
                let mut data: Vec<u8> = buffer.to_vec();
                data.resize(n, 0);
                match parse_request(String::from_utf8_lossy(&data).to_string()) {
                    Ok(parsed) => {
                        let mut response_struct: ResponseStruct =
                            create_response(parsed.clone()).unwrap();
                        response_struct.headers.insert(
                            String::from("Sec-WebSocket-Accept"),
                            generate_accept(
                                parsed.headers.get("Sec-WebSocket-Key").unwrap().clone(),
                            ),
                        );
                        let response_builded: String = build_response(response_struct);
                        response = response_builded;
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(_) => {
                return;
            }
        }
        socket.write(response.as_bytes()).await.unwrap();
    }

    pub async fn run(self) {
        let self_arc: Arc<Self> = Arc::new(self);
        loop {
            let (mut socket, _) = self_arc.listener.accept().await.unwrap();

            self_arc.handshake(socket).await;
        }
    }
}

// pub async fn server(url: &str) {
//     let listener: TcpListener = TcpListener::bind(url).await.unwrap();

//     loop {
//         let (mut socket, _) = listener.accept().await.unwrap();

//         tokio::spawn(async move { loop {} });
//     }
// }
