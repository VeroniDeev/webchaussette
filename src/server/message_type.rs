use crate::frame::frame_types::Opcode;

#[derive(Debug)]
pub enum Types{
    String(String),
    Binary(Vec<u8>)
}

impl Types{
    pub fn from_opcode( opcode: Opcode, data: Vec<u8>) -> Self{
        match opcode {
            Opcode::Text => {
                Self::String(String::from_utf8_lossy(&data).to_string())
            },
            Opcode::Binary => {
                Self::Binary(data)
            },

            // TODO
            _ => {
                unimplemented!()
            }
        }
    }
}

