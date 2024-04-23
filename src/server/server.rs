use crate::{
    frame::frame_types::{Frame, Opcode, PayloadLen},
    handshake::{create_response, parse_request},
    utils::{build_response, generate_accept},
    websocket_types::{ResponseStruct, BUFFER_SIZE},
};
use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
};

use super::{EventHandler, Public, Types};

pub struct Server {
    listener: TcpListener,
    event_listener: Option<Box<dyn EventHandler + Send>>,
}

impl Server {
    pub async fn new(url: &str) -> Self {
        let listener: TcpListener = TcpListener::bind(url).await.unwrap();
        Self {
            listener,
            event_listener: None,
        }
    }

    async fn handshake(&self, socket: Arc<Mutex<TcpStream>>) {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut response: String = String::new();
        let mut socket: MutexGuard<'_, TcpStream> = socket.lock().await;

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
                    // TODO
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

        let mut socket_guard: MutexGuard<'_, TcpStream> = socket.lock().await;
        let socket: &mut TcpStream = &mut *socket_guard;

        loop {
            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let mut data: Vec<u8> = buffer.to_vec();
                    data.resize(n, 0);

                    if size == 0 {
                        frame.default_header(data.clone());
                        if frame.opcode == Opcode::Close {
                            frame.payload_length = PayloadLen::LengthU8(0);
                            let _ = socket.write(&frame.to_bytes()).await;
                            return;
                        }
                        size = TryInto::<usize>::try_into(frame.payload_length.clone()).unwrap();
                    }

                    data_vec.append(&mut data);
                    cur_size += n;
                }
                Ok(_) => {
                    break;
                }
                Err(_) => unimplemented!(),
            }

            if cur_size >= size {
                frame.default_from(data_vec.clone());

                if self.event_listener.is_some() {
                    let event: &Box<dyn EventHandler + Send> =
                        self.event_listener.as_ref().unwrap();
                    let mut public: Public = Public {
                        socket,
                        closed: false,
                        message: Types::from_opcode(frame.opcode, frame.payload_data.unwrap()),
                    };
                    event.on_message(&mut public).await;

                    if public.closed == true {
                        return;
                    }
                }

                cur_size = 0;
                size = 0;
                data_vec.clear();
                frame = Frame::default();
            }
        }
    }

    async fn close(&self, socket: Arc<Mutex<TcpStream>>) {
        if self.event_listener.is_some() {
            let event: &Box<dyn EventHandler + Send> = self.event_listener.as_ref().unwrap();
            event.on_close().await;
        }
        let mut socket: MutexGuard<'_, TcpStream> = socket.lock().await;
        socket.shutdown().await.expect("Close failed");
    }

    pub async fn run(self) {
        let self_arc: Arc<Self> = Arc::new(self);

        loop {
            let (socket, _) = self_arc.listener.accept().await.unwrap();
            let socket_arc: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(socket));

            let self_arc_clone: Arc<Server> = Arc::clone(&self_arc);
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

    pub fn set_handler(&mut self, handler: Box<dyn EventHandler + Send>) {
        self.event_listener = Some(handler);
    }
}
