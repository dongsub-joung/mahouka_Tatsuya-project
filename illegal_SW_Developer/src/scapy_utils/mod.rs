// from scapy.all import AsyncSniffer, sendp, sniff

use std::{thread, time};

use crate::packet::{self, Packet};


// return type 後で書く
pub fn send_recv_with_filter(packet: packet::Packet, filter: String, timeout: usize, iface: String)
-> Vec<packet::Packet>{

    let filter=(filter);
    let iface=iface;
    let sniffer = AsyncSniffer(
        filter,
        iface,
    );

    sniffer.start();

    let verbose= false;
    sendp(packet, iface, verbose);
    
    {
        let timeout = time::Duration::from_millis(timeout as u64);
        thread::sleep(timeout);
    }

    sniffer.stop();

    sniffer.results
}