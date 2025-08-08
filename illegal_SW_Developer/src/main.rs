mod spoofer_config;
mod dhcp_server;
mod dhcp_coerce;
mod dhcp_client;

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
    
    let iface= args.iface;
    let flag= true;
    let target_server= args.target_server;


    let domain_name = args.domain_name;
    // let client_id = get_if_hwaddr(args.iface).replace(":","");
    let fqdn = format!("aaa.{}", domain_name);

    let dhcp_client =dhcp_client::DHCPClient::new(iface, flag, target_server);

    let leased_ip= dhcp_client.dhcp_dora(client_id, fqdn, requested_ip, dhcp_server, max_retry, fqdn_server_flag, relay_address);

    if leased_ip.is_none(){
            println!("[*] Failed to get an IP lease from the server.")
    }else{
            println!("[*] Successfully leased IP {} with FQDN {}. ", leased_ip.into(), fqdn)
    }
}