// from scapy.all import AsyncSniffer, sendp, sniff

use std::{thread, time};
use pcap::{Device, Packet};

use std::io::prelude::*;
use std::net::TcpStream;

use crate::packet::{self, EnhancePacket};


// return type 後で書く
pub fn send_recv_with_filter(packet: packet::PacketOverride, filter: String, timeout: usize, iface: String)
-> Vec<packet::PacketOverride>{

    let filter= filter.clone();
    let iface=iface.clone();
    let verbose= false;

    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();

    let mut packets_v= Vec::new();
    while let Ok(packet) = cap.next_packet() {
        let packet= packet::PacketOverride::new(packet);
        packets_v.push(packet);
    }

    for packet in packets_v {
        let sniffer_results= sendp(packet, iface.clone(), verbose, "");
    }
    
    {
        let timeout = time::Duration::from_millis(timeout as u64);
        thread::sleep(timeout);
    }

    return packets_v;
}

fn sendp(packet :packet::PacketOverride, iface: String, verbose: bool, ip: &'static str) -> std::io::Result<()> {

    let mut stream = TcpStream::connect(ip)?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    
    Ok(())
} 