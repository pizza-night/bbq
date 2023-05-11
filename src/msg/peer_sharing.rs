use crate::header::msg_type::MsgType;
use crate::server::Server;
use std::io;
use std::io::Read;
use std::net::TcpStream;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct PeerSharing {
    pub ips: Vec<IpAddr>,
}

impl PeerSharing {
    pub fn read_from_stream(stream: &mut TcpStream) -> io::Result<Self> {
        let mut packet = PeerSharing { ips: Vec::new() };
        let mut buf = [0; 2];
        match stream.read_exact(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error read from {}", stream.peer_addr().unwrap()),
                ))
            }
        }
        let ipv4_count = u8::from_be(buf[0]);
        let ipv6_count = u8::from_be(buf[1]);
        for _ in 0..ipv4_count {
            let mut buf = [0; 4];
            stream.read_exact(&mut buf)?;
            let ip = IpAddr::V4(buf.into());
            packet.ips.push(ip);
        }
        for _ in 0..ipv6_count {
            let mut buf = [0; 16];
            stream.read_exact(&mut buf)?;
            let ip = IpAddr::V6(buf.into());
            packet.ips.push(ip);
        }
        Ok(packet)
    }

    pub fn create_packet(data: &Vec<IpAddr>) -> Vec<u8> {
        let mut buf = Vec::new();
        let msg_type = MsgType::PeerSharing as u8;
        let mut ipv4_buf = Vec::new();
        let mut ipv6_buf = Vec::new();
        let mut ipv4_count = 0;
        let mut ipv6_count = 0;
        for ip in data {
            match ip {
                std::net::IpAddr::V4(ip) => {
                    ipv4_buf.extend_from_slice(&ip.octets());
                    ipv4_count += 1;
                }
                std::net::IpAddr::V6(ip) => {
                    ipv6_buf.extend_from_slice(&ip.octets());
                    ipv6_count += 1;
                }
            }
        }
        buf.extend_from_slice(&msg_type.to_be_bytes());
        buf.extend_from_slice(&(ipv4_count as u8).to_be_bytes());
        buf.extend_from_slice(&(ipv6_count as u8).to_be_bytes());
        buf.extend_from_slice(&ipv4_buf);
        buf.extend_from_slice(&ipv6_buf);
        buf
    }
}
