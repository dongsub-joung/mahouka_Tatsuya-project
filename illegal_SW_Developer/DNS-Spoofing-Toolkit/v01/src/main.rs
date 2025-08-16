extern crate packet;

use libc::*;
use nfqueue::*;
use std::fmt::Write;
use packet::builder::Builder;
use packet::icmp;

fn process_packet(packet: Vec<u8>){
    
}

fn main() {
    const QUEUE_NUM: usize = 0;

    let (google_dns_host, google_ip)= 
        (b"www.google.com.".to_vec(), "192.168.1.100"); 
    let (facebook_dns_host, facebook_ip)= 
        (b"facebook.com.".to_vec(), "172.217.19.142"); 

    let dns_hosts: Vec<(Vec<u8>, &'static str)>= 
        Vec::from([(google_dns_host, google_ip), (facebook_dns_host, facebook_ip)]);

    { // NetFilterQueue
        let mut q = nfqueue::Queue::new();
        let callback= process_packet();
        
        q.open();

        let rc = q.bind(libc::AF_INET);
        assert!(rc == QUEUE_NUM);

        q.create_queue(QUEUE_NUM, callback);
        q.set_mode(nfqueue::CopyMode::CopyPacket, 0xffff);

        q.set_callback(callback);
        q.run_loop();

        q.close();
    }


    ipt.delete_chain("nat", "NEWCHAINNAME")
}