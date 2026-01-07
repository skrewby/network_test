use crate::builder::{Layer, PacketBuilder};

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum Ethertype {
    IPV4 = 0x0800,
}

pub struct Ethernet {
    builder: PacketBuilder,
    dst_mac: [u8; 6],
    src_mac: [u8; 6],
    ethertype: Ethertype,
}

impl Ethernet {
    pub fn new(dst_mac: [u8; 6], src_mac: [u8; 6], ethertype: Ethertype) -> Self {
        Ethernet {
            builder: PacketBuilder::new(),
            dst_mac,
            src_mac,
            ethertype,
        }
    }

    pub fn layer<L: Layer + 'static>(mut self, layer: L) -> Self {
        self.builder = self.builder.layer(layer);
        self
    }

    pub fn build(self, data: Vec<u8>) -> Vec<u8> {
        let dst_mac = self.dst_mac;
        let src_mac = self.src_mac;
        let ethertype = self.ethertype;
        let packet = self.builder.build(data);

        let mut frame = Vec::new();

        // MAC destination (6 bytes)
        frame.extend_from_slice(&dst_mac);

        // MAC source (6 bytes)
        frame.extend_from_slice(&src_mac);

        // Ethertype (2 bytes)
        frame.extend_from_slice(&(ethertype as u16).to_be_bytes());

        // Payload (42 - 1500 bytes)
        frame.extend_from_slice(&packet);

        frame
    }
}
