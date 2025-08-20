use std::{thread, time::Duration};

use mac_address;
use ipconfig;
use pnet_packet::arp;

fn spoof(){

}

fn restore(){

}

fn selete_adapter(ip_addrs: Vec<?>) -> ? {

}

fn main() {
    let mac_addr= mac_address::get_mac_address().unwrap_or_else(|e| None);

    let ip_addrs= {
        let mut v= Vec::new();

        for adapter in ipconfig::get_adapters()? {
            v.push(adapter.ip_addresses());
        }
        v
    };

    if mac_addr.is_none() {
        println!("Unknown OS or Unable OS");
    }else{
        let selected_ip= selete_adapter(ip_addrs);
        
        // null is handled by is_none(Always true)
        let mac_addr= mac_addr.unwrap();

        loop {    
            // @Todo conn
            // 1. host
            spoof();
            // 2. target
            spoof();

            thread::sleep(Duration::from_secs(1));
        }

        // restore conn
        restore();
    }
}
