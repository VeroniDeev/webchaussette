use crate::{
    frame::frame_types::{Frame, Opcode},
    handshake::{create_response, parse_request},
    utils::{build_response, generate_accept},
    websocket_types::ResponseStruct,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    listener: TcpListener,
    incomplete_data: Arc<Mutex<HashMap<TcpStream, String>>>,
}

impl Server {
    pub async fn new(url: &str) -> Self {
        let listener: TcpListener = TcpListener::bind(url).await.unwrap();
        Self {
            listener,
            incomplete_data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn handshake(&self, socket: Arc<Mutex<TcpStream>>) {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut response: String = String::new();
        let mut socket: MutexGuard<'_, TcpStream> = socket.lock().unwrap();

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

    async fn receive_data(&self, socket: Arc<Mutex<TcpStream>>) {
        let mut socket: MutexGuard<'_, TcpStream> = socket.lock().unwrap();
        let mut buffer: [u8; 1024] = [0; 1024];

        loop {
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    let mut data = buffer.to_vec();
                    data.resize(n, 0);
                    let mut frame: Frame = Frame::default();
                    frame.default_from(data);
                    if frame.opcode == Opcode::Close {
                        return;
                    }
                }
                Err(_) => {}
            }
        }
    }

    async fn close(&self, socket: Arc<Mutex<TcpStream>>) {
        let mut socket: MutexGuard<'_, TcpStream> = socket.lock().unwrap();
        socket.shutdown().await.expect("Close failed");
    }

    pub async fn run(self) {
        let self_arc: Arc<Self> = Arc::new(self);

        loop {
            let (socket, _) = self_arc.listener.accept().await.unwrap();
            let socket_arc: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(socket));

            self_arc.clone().handshake(socket_arc.clone()).await;
            self_arc.clone().receive_data(socket_arc.clone()).await;
            self_arc.close(socket_arc).await;
        }
    }
}
