pub mod cli_args;
pub mod client;
pub mod header;
pub mod msg;
pub mod server;
use clap::Parser;
use cli_args::Args;
use server::Server;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut server = Server::new(args.port)?;
    let mut server_clone = server.clone();
    let server_thread = thread::spawn(move || match server_clone.start() {
        Ok(_) => println!("Server exited"),
        Err(e) => println!("Error: {}", e),
    });
    let server_clone = server.clone();
    let client = thread::spawn(move || match client::write_to_chat(server_clone) {
        Ok(_) => println!("Client exited"),
        Err(e) => println!("Error: {}", e),
    });
    match server.connect_to_peers(args.peers) {
        Ok(_) => {}
        Err(_) => println!("Error connecting"),
    }
    client.join().unwrap();
    server_thread.join().unwrap();
    Ok(())
}
