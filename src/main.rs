use crate::{ethernet::Ethernet, ip::IP, serial::SerialConnection, slip::SLIP, udp::UDP};
use clap::{Parser, ValueEnum};

mod builder;
mod ethernet;
mod ip;
mod serial;
mod slip;
mod udp;

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Datalink {
    Ethernet,
    Slip,
}

impl std::fmt::Display for Datalink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Datalink::Ethernet => write!(f, "ethernet"),
            Datalink::Slip => write!(f, "slip"),
        }
    }
}

/// Utility to send IP/UDP data over either Ethernet or SLIP
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Layer 2 (Datalink) mode to use
    #[arg(short, long, default_value_t = Datalink::Slip)]
    datalink: Datalink,

    /// Print packet hex representation to stdout
    #[arg(long, default_value_t = false)]
    print: bool,

    /// Serial port file descriptor
    #[arg(short, long, default_value = "dev/ttyUSB0")]
    port: String,

    /// Serial port baud rate
    #[arg(short, long, default_value_t = 115200)]
    baud: u32,
}

fn main() {
    let args = Args::parse();

    let data = vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x7, 0xAB, 0xCD, 0xEF];

    let src_addr = [192, 168, 1, 20];
    let src_port = 5000;

    let dst_addr = [192, 168, 20, 10];
    let dst_port = 8080;

    let dst_mac = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let src_mac = [0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];

    let packet = match args.datalink {
        Datalink::Slip => SLIP::new()
            .layer(IP::new(src_addr, dst_addr, ip::Protocol::UDP))
            .layer(UDP::new(src_port, dst_port))
            .build(data),
        Datalink::Ethernet => Ethernet::new(dst_mac, src_mac, ethernet::Ethertype::IPV4)
            .layer(IP::new(src_addr, dst_addr, ip::Protocol::UDP))
            .layer(UDP::new(src_port, dst_port))
            .build(data),
    };

    if args.print {
        println!("Packet: {}", hex_string(&packet));
    }

    let mut serial = match SerialConnection::connect(&args.port, args.baud) {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error when opening serial port: {}", e);
            std::process::exit(1);
        }
    };
    serial
        .write(&packet)
        .expect("Error when writing to serial port");
}

fn hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}
