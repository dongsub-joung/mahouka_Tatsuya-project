use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    #[cfg(not(any(target_os = "windows")))]
    {
        println!("Running on an un-windows operating system!");

        // about conn info
        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let local_port= 5000;
        let remote_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let remote_port= 5000;

        // connection


        // check keeping conn
        // mouse pointer x,y + screenshot copy
        // shift + cnt + print-f(gnome)
    }

    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows!");
        // Add Windows-specific functionality here.
    }
}