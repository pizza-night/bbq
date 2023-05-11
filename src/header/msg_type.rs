#[repr(u8)]
pub enum MsgType {
    Data = 0,
    SetUsername = 1,
    PeerSharing = 2,
}

impl From<u8> for MsgType {
    fn from(orig: u8) -> Self {
        match orig {
            0 => MsgType::Data,
            1 => MsgType::SetUsername,
            2 => MsgType::PeerSharing,
            _ => unreachable!(),
        }
    }
}
