use crate::header::msg_type::MsgType;
use std::io;
use std::io::Read;
use std::net::TcpStream;

pub struct SetUsername {}

impl SetUsername {
    pub fn read_username(stream: &mut TcpStream) -> io::Result<String> {
        let mut buf = [0; 1];
        match stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error read from {}", stream.peer_addr().unwrap()),
                ))
            }
        }
        let size = u8::from_be_bytes(buf);
        let mut buf = vec![0; size as usize];
        stream.read_exact(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(s) => Ok(s),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }

    pub fn create_packet(username: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        let msg_type = MsgType::SetUsername as u8;
        buf.extend_from_slice(&msg_type.to_be_bytes());
        buf.extend_from_slice(&(username.len() as u8).to_be_bytes());
        buf.extend_from_slice(username.as_bytes());
        buf
    }
}
