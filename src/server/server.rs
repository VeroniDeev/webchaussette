use crate::{
    frame::{
        self,
        frame_types::{Frame, Opcode},
    },
    handshake::{create_response, parse_request},
    utils::{build_response, generate_accept, unmask_payload},
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
}

impl Server {
    pub async fn new(url: &str) -> Self {
        let listener: TcpListener = TcpListener::bind(url).await.unwrap();
        Self { listener }
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
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut size: usize = 0;
        let mut data_vec: Vec<u8> = Vec::new();
        let mut frame: Frame = Frame::default();
        let mut cur_size: usize = 0;

        let mut socket_guard = socket.lock().await;
        let socket = &mut *socket_guard;

        loop {
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    let mut data = buffer.to_vec();
                    data.resize(n, 0);

                    if size == 0 {
                        frame.default_header(data.clone());
                        size = TryInto::<usize>::try_into(frame.payload_length.clone()).unwrap();
                    }

                    data_vec.append(&mut data);
                    cur_size += n;
                }
                Err(_) => unimplemented!(),
            }

            if cur_size >= size {
                frame.default_from(data_vec.clone());

                println!(
                    "{:?}",
                    String::from_utf8_lossy(&frame.payload_data.unwrap())
                );

                cur_size = 0;
                size = 0;
                data_vec.clear();
                frame = Frame::default();
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
