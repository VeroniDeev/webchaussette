use crate::{
    handshake::{create_response, parse_request},
    utils::{build_response, generate_accept},
};
use std::error::Error;

use bytes::Bytes;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub async fn server(url: &str) {
    let listener = TcpListener::bind(url).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer: [u8; 1024] = [0; 1024];

            loop {
                let mut response: String = String::new();
                match socket.read(&mut buffer).await {
                    Ok(n) => {
                        let mut data: Vec<u8> = buffer.to_vec();
                        data.resize(n, 0);
                        match parse_request(String::from_utf8_lossy(&data).to_string()) {
                            Ok(parsed) => {
                                let mut response_struct = create_response(parsed.clone()).unwrap();
                                response_struct.headers.insert(
                                    String::from("Sec-WebSocket-Accept"),
                                    generate_accept(
                                        parsed.headers.get("Sec-WebSocket-Key").unwrap().clone(),
                                    ),
                                );
                                let response_builded = build_response(response_struct);
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
        });
    }
}
