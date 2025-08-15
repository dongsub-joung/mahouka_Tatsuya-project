use libc::*;
use nfqueue::*;
use std::fmt::Write;

fn main() {
    const QUEUE_NUM: usize = 0;

    let (google_dns_host, google_ip)= 
        (b"www.google.com.".to_vec(), "192.168.1.100"); 
    let (facebook_dns_host, facebook_ip)= 
        (b"facebook.com.".to_vec(), "172.217.19.142"); 

    let dns_hosts: Vec<(Vec<u8>, &'static str)>= 
        Vec::from([(google_dns_host, google_ip), (facebook_dns_host, facebook_ip)]);
    
    { // iptable command
        let iptable_command= 
            format!("iptables -I FORWARD -j NFQUEUE --queue-num {}", QUEUE_NUM);
        let ipt = iptables::new(false).unwrap();
        ipt.new_chain("nat", "NEWCHAINNAME");
        ipt.append("nat", "NEWCHAINNAME", &iptable_command);
    }

    { // NetFilterQueue
        let mut q = nfqueue::Queue::new();

        q.open();

        let rc = q.bind(libc::AF_INET);
        assert!(rc == 0);

        q.create_queue(0, callback);
        q.set_mode(nfqueue::CopyMode::CopyPacket, 0xffff);

        q.set_callback(callback);
        q.run_loop();

        q.close();
    }
}