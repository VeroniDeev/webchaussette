#[derive(Debug)]
pub enum Opcode {
    Continuation,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
    Unknow,
}

impl Opcode {
    fn with_bits(data: [u8; 4]) -> Self {
        let mut result = 0;
        for element in data {
            result = (result << 8) | element as u32;
        }

        match result {
            0 => Self::Continuation,
            1 => Self::Text,
            2 => Self::Binary,
            8 => Self::Close,
            9 => Self::Ping,
            10 => Self::Pong,
            _ => Self::Unknow,
        }
    }
}

#[derive(Debug)]
pub enum PayloadLen {
    LengthU8(u8),
    LengthU16(u16),
    LengthU64(u64),
    Unknow,
}

impl PayloadLen {
    fn with_size(data: [u8; 7]) -> Self {
        let mut result: u64 = 0;
        for bit in data {
            result = (result << 1) | u64::from(bit);
        }

        result = result & 0xFF;

        match result {
            126 => Self::LengthU16(0),
            127 => Self::LengthU64(0),
            _ => Self::LengthU8(result as u8),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub is_final: bool,
    pub rsv1: bool,
    pub rsv2: bool,
    pub rsv3: bool,
    pub opcode: Opcode,
    pub mask: bool,
    pub payload_length: PayloadLen,
    pub masking_key: Option<[u8; 4]>,
    pub payload_data: Vec<u8>,
}

impl Frame {
    pub fn default_from(&mut self, data: Vec<u8>) {
        let first_octal: u8 = data[0];
        let second_octal: u8 = data[1];
        let mut masking_array: Vec<u8> = Vec::new();

        let mut first_bits: [u8; 8] = [0; 8];
        let mut second_bits: [u8; 8] = [0; 8];

        for i in 0..8 {
            first_bits[i] = (first_octal >> (7 - i)) & 1;
            second_bits[i] = (second_octal >> (7 - i)) & 1;
        }

        self.is_final = first_bits[0] != 0;
        self.rsv1 = first_bits[1] != 0;
        self.rsv2 = first_bits[2] != 0;
        self.rsv3 = first_bits[3] != 0;
        self.opcode = Opcode::with_bits(first_bits[4..8].try_into().unwrap());

        self.mask = second_bits[0] != 0;
        let payload_len: PayloadLen = PayloadLen::with_size(second_bits[1..8].try_into().unwrap());

        match payload_len {
            PayloadLen::LengthU8(_) => {
                self.payload_length = payload_len;
                masking_array = data[3..7].try_into().unwrap();
                self.payload_data = data[8..].try_into().unwrap();
            }

            PayloadLen::LengthU16(_) => {
                let length_array: &[u8] = &data[2..4];
                masking_array = data[5..9].try_into().unwrap();
                self.payload_data = data[10..].try_into().unwrap();
                let binary_length: String = format!("{:b}{:b}", length_array[0], length_array[1]);
                let length: u16 = u16::from_str_radix(&binary_length, 2).unwrap();
                self.payload_length = PayloadLen::LengthU16(length);
            }

            // TODO
            PayloadLen::LengthU64(_) => {
                unimplemented!();
            }

            // TODO
            _ => {}
        }

        if self.mask {
            self.masking_key = Some(masking_array.try_into().unwrap());
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            is_final: true,
            rsv1: false,
            rsv2: false,
            rsv3: false,
            opcode: Opcode::Unknow,
            mask: false,
            payload_length: PayloadLen::Unknow,
            masking_key: None,
            payload_data: Vec::new(),
        }
    }
}
