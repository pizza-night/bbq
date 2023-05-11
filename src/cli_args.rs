use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub name: String,
    #[arg(long)]
    pub peers: Vec<IpAddr>,
    #[arg(short, long, default_value = "2504")]
    pub port: u16,
}
