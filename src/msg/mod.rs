pub mod data;
pub mod peer_sharing;
pub mod set_username;
use crate::header::msg_type::MsgType;
use crate::header::Header;
use crate::msg::data::Data;
use crate::msg::peer_sharing::PeerSharing;
use crate::server::Server;
use std::io;
use std::io::Read;
use std::mem::size_of;
use std::net::IpAddr;

pub fn handle_msg(ip: IpAddr, mut server: Server) -> io::Result<()> {
    // Handle multiple access stream
    let mut buf = [0; size_of::<Header>()];
    let mut username = ip.to_string();
    let mut stream = server
        .streams
        .lock()
        .unwrap()
        .get(&ip)
        .unwrap()
        .try_clone()?;
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                dbg!("Connection closed");
                // connection closed
                return Ok(());
            }
            Ok(_) => {}
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error read from {}", username),
                ));
            }
        }
        let header = Header::from_bytes(&buf)?;
        match header.msg_type {
            MsgType::Data => {
                let data = Data::read_from_stream(&mut stream, &username)?;
                data.handle();
            }
            MsgType::SetUsername => {
                let old_username = username.clone();
                username = set_username::SetUsername::read_username(&mut stream)?;
                println!("{} change username to {}", old_username, username);
            }
            MsgType::PeerSharing => {
                let peer_sharing = PeerSharing::read_from_stream(&mut stream)?;
                let _ = server.connect_to_peers(peer_sharing.ips.clone());
            }
        }
    }
    // success value
    //Ok(())
}
