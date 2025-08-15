mod spoofer_config;
mod dhcp_server;
mod dhcp_coerce;
mod dhcp_client;
mod utils;
mod scapy_utils;
mod packet;
mod bootp;

use clap::Parser;
use mac_address::get_mac_address;
use std::net::{IpAddr, Ipv4Addr};


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
    
    let iface= args.iface;
    let flag= true;
    let target_server= args.target_server;

    let domain_name = args.domain_name;
    let fqdn = format!("aaa.{}", domain_name);
    
    // let client_id = get_if_hwaddr(args.iface).replace(":","");
    let client_id = match get_mac_address() {
        Ok(Some(ma)) => { ma },
        _ => { panic!() },
    };

    let dhcp_client =dhcp_client::DHCPClient::new(iface, flag, target_server);

    //  a specific IP address to request from the DHCP server
    let requested_ip= IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // Optional. The specific DHCP server address to target. Without it, a broadcast is sent\n
        // and the first server to reply would be used.
    let dhcp_server= IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let max_retry= 1024;
    
    let fqdn_server_flag= false;

    let relay_address= IpAddr::V4(Ipv4Addr::new(127,0,0,1));

    let leased_ip= dhcp_client.dhcp_dora(
        client_id, fqdn.clone(), requested_ip, dhcp_server, max_retry, fqdn_server_flag, relay_address
    );

    if leased_ip.is_none(){
        println!("[*] Failed to get an IP lease from the server.")
    }else{
        let mut raw_data= None;
        if let Some(str) = raw_data {
            return str;  
        }

        println!("[*] Successfully leased IP {:?} with FQDN {}. ", raw_data, fqdn)
    }
}