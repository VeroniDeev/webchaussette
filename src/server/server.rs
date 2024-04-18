use crate::{
    frame::frame_types::{Frame, Opcode},
    handshake::{create_response, parse_request},
    utils::{build_response, generate_accept},
    websocket_types::{ResponseStruct, BUFFER_SIZE},
};
use std::{collections::HashMap, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
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
        let mut socket = socket.lock().await;

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
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut frame: Frame = Frame::default();
        let mut size: usize = 0;
        let mut cur_size: usize = 0;

        let mut socket: MutexGuard<'_, TcpStream> = socket.lock().await;
        loop {
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    let mut data = buffer.to_vec();
                    data.resize(n, 0);
                    if size == 0 {
                        frame.default_from(data.clone());
                        size = TryInto::<usize>::try_into(frame.payload_length.clone()).unwrap();
                        println!("{:?}", size);

                        if frame.opcode == Opcode::Close {
                            return;
                        }
                    } else if cur_size < size {
                        frame.payload_data.append(&mut data);
                        println!("{}", frame.payload_data.len());
                    } else if cur_size >= size {
                        cur_size = 0;
                        size = 0;
                    }
                    cur_size += n;
                }
                Err(_) => {}
            }
        }
    }

    async fn close(&self, socket: Arc<Mutex<TcpStream>>) {
        let mut socket: MutexGuard<'_, TcpStream> = socket.lock().await;
        socket.shutdown().await.expect("Close failed");
    }

    pub async fn run(self) {
        let self_arc: Arc<Self> = Arc::new(self);

        loop {
            let (socket, _) = self_arc.listener.accept().await.unwrap();
            let socket_arc: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(socket));

            let self_arc_clone = Arc::clone(&self_arc);
            let _ = tokio::spawn(async move {
                self_arc_clone.clone().handshake(socket_arc.clone()).await;
                self_arc_clone
                    .clone()
                    .receive_data(socket_arc.clone())
                    .await;
                self_arc_clone.close(socket_arc).await;
            });
        }
    }
}
