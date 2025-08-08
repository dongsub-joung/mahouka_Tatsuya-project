mod spoofer_config;
mod dhcp_server;
mod dhcp_coerce;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    iface: String,
    domain_name: String,
    target_server: String,
    coere_ip: String,
    relay_ip: String,
}

fn main() {
    let args = Args::parse();
}