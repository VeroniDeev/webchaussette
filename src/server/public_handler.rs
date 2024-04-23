use std::{error::Error, sync::Arc};

use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{Mutex, MutexGuard},
};

use crate::frame::frame_types::{Frame, Opcode, PayloadLen};

use super::Types;

#[derive(Debug)]
pub struct Public<'a> {
    pub socket: &'a mut TcpStream,
    pub message: Types,
    pub closed: bool,
}

impl<'a> Public<'a> {
    pub async fn send_string(&mut self, message: String) -> Result<(), Box<dyn Error>> {
        let message_vec: Vec<u8> = message.as_bytes().to_vec();

        let mut frame: Frame = Frame::default();
        frame.opcode = Opcode::Text;
        frame.payload_length = PayloadLen::from_size(message_vec.len());
        frame.payload_data = Some(message_vec);

        match self.socket.write(&frame.to_bytes()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub async fn close(&mut self) {
        self.closed = true;
    }

    pub async fn send_binary(&mut self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let mut frame: Frame = Frame::default();
        frame.opcode = Opcode::Binary;
        frame.payload_length = PayloadLen::from_size(data.len());
        frame.payload_data = Some(data);

        match self.socket.write(&frame.to_bytes()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}
