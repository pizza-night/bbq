pub mod cli_args;
pub mod client;
pub mod header;
pub mod msg;
pub mod server;
use clap::Parser;
use cli_args::Args;
use server::Server;
use std::net::SocketAddr;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut server = Server::new(args.port)?;
    let server_clone = server.clone();
    let mut yet_another_server_clone = server.clone();
    let server_thread = thread::spawn(move || {
        server.start();
    });
    let client = thread::spawn(move || {
        client::write_to_chat(server_clone);
    });
    for peer in args.peers {
        let sockaddr = SocketAddr::from(peer);
        yet_another_server_clone.connect_to_peers(vec![sockaddr]);
    }
    client.join().unwrap();
    server_thread.join().unwrap();
    Ok(())
}
