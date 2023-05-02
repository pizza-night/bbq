pub mod data;
use crate::header::msg_type::MsgType;
use crate::header::Header;
use crate::msg::data::Data;
use std::io;
use std::io::Read;
use std::mem::size_of;
use std::net::TcpStream;

pub fn handle_msg(mut stream: TcpStream) -> io::Result<()> {
    // Handle multiple access stream
    let mut buf = [0; size_of::<Header>()];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                // connection closed
                return Ok(());
            }
            Ok(_) => {}
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error read from {}", stream.peer_addr().unwrap()),
                ))
            }
        }
        let header = Header::from_bytes(&buf)?;
        match header.msg_type {
            MsgType::Data => {
                let data = Data::read_from_stream(&mut stream)?;
                data.handle();
            }
        }
    }
    // success value
    //Ok(())
}
