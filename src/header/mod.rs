pub mod msg_type;

pub struct Header {
    pub msg_type: msg_type::MsgType,
}

impl Header {
    pub fn from_bytes(bytes: &[u8]) -> std::io::Result<Header> {
        if bytes.len() != 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid header",
            ));
        }
        let msg_type = u8::from_be_bytes([bytes[0]]);
        Ok(Header {
            msg_type: msg_type.into(),
        })
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            msg_type: msg_type::MsgType::Data,
        }
    }
}
