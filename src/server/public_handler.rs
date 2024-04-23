use std::{error::Error, sync::Arc};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::{Mutex, MutexGuard}};

use crate::frame::frame_types::Frame;

use super::Types;

#[derive(Debug)]
pub struct Public{
    pub socket: Arc<Mutex<TcpStream>>,
    pub message: Types,
    pub closed: bool
}

impl Public{
    pub async fn send_string(&mut self, message: String) -> Result<(), Box<dyn Error>>{
        let mut socket_guard: MutexGuard<'_, TcpStream> = self.socket.lock().await;
        let socket: &mut TcpStream = &mut *socket_guard;
        
        let message_vec = message.as_bytes().to_vec();
        let mut frame: Frame = Frame::default();
        frame.default_header(message_vec.clone());
        frame.default_from(message_vec);

        match socket.write(&frame.to_bytes()).await {
            Ok(_) => {
                Ok(())
            },
            Err(err) => Err(Box::new(err))
        }
    }

    pub async fn close(&mut self){
        self.closed = true;
    }

    // TODO
    pub async fn send_binary(&mut self, data: Vec<u8>){
        unimplemented!()
    }
}