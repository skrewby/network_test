use std::io::{self, Write};
use std::time::Duration;

use serialport::SerialPort;

pub struct SerialConnection {
    connection: Box<dyn SerialPort>,
}

impl SerialConnection {
    pub fn connect(port: &str, baud_rate: u32) -> Result<SerialConnection, io::Error> {
        let connection = serialport::new(port, baud_rate)
            .timeout(Duration::from_millis(1000))
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .open()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        connection
            .clear(serialport::ClearBuffer::All)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        std::thread::sleep(Duration::from_millis(500));

        Ok(SerialConnection { connection })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
        self.connection
            .write_all(data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        self.connection.flush()?;

        Ok(data.len())
    }

    pub fn read(&mut self) -> Option<Vec<u8>> {
        let mut data = Vec::new();
        let mut buffer: Vec<u8> = vec![0; 1000];

        loop {
            match self.connection.read(buffer.as_mut_slice()) {
                Ok(0) => break,
                Ok(t) => data.extend_from_slice(&buffer[..t]),
                Err(_) => break,
            }
        }

        match data.len() {
            0 => None,
            _ => Some(data),
        }
    }
}
