use std::net::Ipv4Addr;

mod utils;
mod tcp_middlebox;
mod mdns;
mod cldap;

struct Net{
    ip: Ipv4Addr,
    times: usize,
}
impl Net {
    pub fn new() -> Self{
        Self { ip: Ipv4Addr::new(196, 0, 0, 1), times: 0 }
    }
    pub fn set_target(ip: String, times: usize) -> Self{
        let ip_chars: Ipv4Addr= ip.trim().parse().unwrap();

        Self { ip: Ipv4Addr::from(ip_chars), times }
    }
}
fn main(){
    utils::show_logo();
    let mut target_info= Net::new();
    loop {
        println!("1. Set the target Infomation and attack count");
        println!("2. MDSN protocol attack");
        println!("3. TCP middlebox reflection attack");
        println!("4. CLDAP reflection attack");
        println!("ip:{}", target_info.ip);
        println!("times:{}", target_info.times);
        println!("99. Exit");
        
        let option_number_str= utils::get_input_data().unwrap_or("99".to_string());
        let option_number: usize= option_number_str.parse().unwrap_or(0);
        match option_number {
            0 => continue,
            1 => {
                println!("IP");
                let ip= utils::get_input_data().unwrap_or("196 0 0 1".to_string()).trim().to_string();
                println!("times");
                let times: usize= utils::get_input_data().unwrap_or("0".to_string())
                    .trim().parse().unwrap();
                target_info= Net::set_target(ip, times);
            }
            2 => mdns::init(),
            3 => tcp_middlebox::init(),
            4 => cldap::init(),
            99 => break,
            _ => continue
        }
    }
}

