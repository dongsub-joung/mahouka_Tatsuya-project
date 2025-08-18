use std::{thread, time::Duration};

use mac_address;
fn spoof(){

}

fn restore(){

}

fn main() {
    let mac_addr= mac_address::get_mac_address().unwrap_or_else(|e| None);

    if mac_addr.is_none() {
        println!("Unknown OS or Unable OS");
    }else{
        loop {
            // null is handled by is_none(Always true)
            let mac_addr= mac_addr.unwrap();

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
