use crate::builder::{Layer, PacketBuilder};

const SLIP_END: u8 = 0xC0;
const SLIP_ESC: u8 = 0xDB;
const SLIP_ESC_END: u8 = 0xDC;
const SLIP_ESC_ESC: u8 = 0xDD;

pub struct SLIP {
    builder: PacketBuilder,
}

impl SLIP {
    pub fn new() -> Self {
        SLIP {
            builder: PacketBuilder::new(),
        }
    }

    pub fn layer<L: Layer + 'static>(mut self, layer: L) -> Self {
        self.builder = self.builder.layer(layer);
        self
    }

    pub fn build(self, data: Vec<u8>) -> Vec<u8> {
        let packet = self.builder.build(data);
        encode(&packet)
    }
}

fn encode(data: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(data.len() + 2);

    encoded.push(SLIP_END);

    for &byte in data {
        match byte {
            SLIP_END => {
                encoded.push(SLIP_ESC);
                encoded.push(SLIP_ESC_END);
            }
            SLIP_ESC => {
                encoded.push(SLIP_ESC);
                encoded.push(SLIP_ESC_ESC);
            }
            _ => {
                encoded.push(byte);
            }
        }
    }

    encoded.push(SLIP_END);

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding() {
        let input: Vec<u8> = vec![0x0A, SLIP_ESC, 0x0B, SLIP_END, 0x0C];
        let expected: Vec<u8> = vec![
            SLIP_END,
            0x0A,
            SLIP_ESC,
            SLIP_ESC_ESC,
            0x0B,
            SLIP_ESC,
            SLIP_ESC_END,
            0x0C,
            SLIP_END,
        ];
        let result = encode(&input);
        assert_eq!(result, expected);
    }
}
