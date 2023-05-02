#[repr(u8)]
pub enum MsgType {
    Data,
}

impl From<u8> for MsgType {
    fn from(orig: u8) -> Self {
        match orig {
            0 => MsgType::Data,
            _ => unimplemented!(),
        }
    }
}
