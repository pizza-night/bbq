use crate::msg::data::Data;
use crate::server::Server;

pub fn write_to_chat(mut server: Server) -> std::io::Result<()> {
    let mut buffer = String::new();
    loop {
        std::io::stdin().read_line(&mut buffer)?;
        server.broadcast(Data::create_packet(buffer.as_str()).as_slice());
        buffer.clear();
    }
}
