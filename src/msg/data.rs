use crate::header::msg_type::MsgType;
use std::io;
use std::io::Read;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Data {
    pub source: String,
    pub data: String,
}

impl Data {
    pub fn read_from_stream(stream: &mut TcpStream) -> io::Result<Data> {
        let mut buf = [0; 4];
        match stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error read from {}", stream.peer_addr().unwrap()),
                ))
            }
        }
        let size = u32::from_be_bytes(buf);
        let mut buf = vec![0; size as usize];
        stream.read_exact(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(s) => Ok(Data {
                data: s,
                source: stream.peer_addr().unwrap().to_string(),
            }),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }

    pub fn handle(&self) {
        println!("{}: {}", self.source, self.data);
    }

    pub fn create_packet(data: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        let msg_type = MsgType::Data as u8;
        buf.extend_from_slice(&msg_type.to_be_bytes());
        buf.extend_from_slice(&(data.len() as u32).to_be_bytes());
        buf.extend_from_slice(data.as_bytes());
        buf
    }
}
