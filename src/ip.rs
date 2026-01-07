use crate::builder::Layer;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Protocol {
    UDP = 17,
}

pub struct IP {
    version: u8,
    ihl: u8,
    dscp: u8,
    ecn: u8,
    ttl: u8,
    protocol: Protocol,
    src_addr: [u8; 4],
    dst_addr: [u8; 4],
}

impl IP {
    pub fn new(src_addr: [u8; 4], dst_addr: [u8; 4], protocol: Protocol) -> Self {
        IP {
            version: 4,
            ihl: 5,
            dscp: 0,
            ecn: 0,
            ttl: 64,
            protocol,
            src_addr,
            dst_addr,
        }
    }
}

fn calculate_checksum(header: &mut [u8]) {
    let mut sum: u32 = 0;

    // Initial addition
    for i in (0..header.len()).step_by(2) {
        sum += ((header[i] as u32) << 8) | (header[i + 1] as u32);
    }

    // Carry addition
    let mut carry_bits = sum >> 16;
    while carry_bits != 0 {
        sum = carry_bits + (sum & 0xFFFF);
        carry_bits = sum >> 16;
    }

    // One's complement
    let checksum = !sum as u16;

    header[10] = (checksum >> 8) as u8;
    header[11] = (checksum & 0xFF) as u8;
}

impl Layer for IP {
    fn wrap(&self, data: &[u8]) -> Vec<u8> {
        let mut packet = Vec::new();
        let total_length = 20 + data.len();

        // Version: 4 bits
        // Internet Header Length (IHL): 4 bits
        packet.push((self.version << 4) | self.ihl);

        // Differentiated Services Code Point (DSCP): 6 bits
        // Explicit Congestion Notification (ECN): 2 bits
        packet.push((self.dscp << 2) | self.ecn);

        // Total Length: 16 bits = Header (20 bytes) + Data (n bytes)
        packet.extend_from_slice(&(total_length as u16).to_be_bytes());

        // Identification: 16 bits
        packet.extend_from_slice(&0u16.to_be_bytes());

        // Flags: 3 bits
        // Fragment Offset: 13 bits
        packet.extend_from_slice(&0u16.to_be_bytes());

        // Time to live (TTL): 8 bits
        packet.push(self.ttl);

        // Protocol: 8 bits
        packet.push(self.protocol as u8);

        // Header Checksum: 16 bits
        packet.extend_from_slice(&0u16.to_be_bytes());

        // Source address: 32 bits
        packet.extend_from_slice(&self.src_addr);

        // Destination address: 32 bits
        packet.extend_from_slice(&self.dst_addr);

        // Options: 0 - 320 bits, padded to multiples of 32 bits
        // Skip - N/A

        calculate_checksum(&mut packet);

        packet.extend_from_slice(data);

        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum() {
        let mut header = vec![
            0x45, 0x00, 0x00, 0x73, 0x00, 0x00, 0x40, 0x00, 0x40, 0x11, 0x00, 0x00, 0xc0, 0xa8,
            0x00, 0x01, 0xc0, 0xa8, 0x00, 0xc7,
        ];
        calculate_checksum(&mut header);
        assert_eq!(header[10], 0xb8);
        assert_eq!(header[11], 0x61);
    }
}
