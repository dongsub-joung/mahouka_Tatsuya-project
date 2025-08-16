mod queue_overroding;

use nfq::{Queue, Verdict};

use crate::queue_overroding::EnhanceQueue;

fn main() -> std::io::Result<()> {
    const QUEUE_NUM: u16 = 0;

    let (google_dns_host, google_ip)= 
        (b"www.google.com.".to_vec(), "192.168.1.100"); 
    let (facebook_dns_host, facebook_ip)= 
        (b"facebook.com.".to_vec(), "172.217.19.142"); 

    let dns_hosts: Vec<(Vec<u8>, &'static str)>= 
        Vec::from([(google_dns_host, google_ip), (facebook_dns_host, facebook_ip)]);

    {
        let mut queue: Queue= Queue::open()?;

        // @Todo use bind function some modified payloads
        let overrided_queue= queue_overroding::QueueOverride::new(queue);
        queue.bind(QUEUE_NUM)?;

        loop {
            let mut msg = queue.recv()?;
            msg.set_verdict(Verdict::Accept);
            queue.verdict(msg)?;
        }
    }

    Ok(())
}