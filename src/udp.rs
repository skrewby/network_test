use crate::builder::Layer;

pub struct UDP {
    src_port: u16,
    dst_port: u16,
}

impl UDP {
    pub fn new(src_port: u16, dst_port: u16) -> UDP {
        UDP { src_port, dst_port }
    }
}

impl Layer for UDP {
    fn wrap(&self, data: &[u8]) -> Vec<u8> {
        let mut packet = Vec::new();
        let length = 8 + data.len();

        // Source Port: 16 bits
        packet.extend_from_slice(&self.src_port.to_be_bytes());

        // Destination Port: 16 bits
        packet.extend_from_slice(&self.dst_port.to_be_bytes());

        // Length: 16 bits = UDP header (8 bytes) + Data Length (n bytes)
        packet.extend_from_slice(&(length as u16).to_be_bytes());

        // Checksum: 16 bits (Optional for IPv4)
        packet.extend_from_slice(&0u16.to_be_bytes());

        // Data: Variable
        packet.extend_from_slice(data);

        packet
    }
}
