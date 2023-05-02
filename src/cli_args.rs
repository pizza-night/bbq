use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub name: String,
    #[arg(long)]
    pub peers: Vec<SocketAddr>,
    #[arg(short, long, default_value = "2504")]
    pub port: u16,
}
