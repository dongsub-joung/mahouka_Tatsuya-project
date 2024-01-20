
mod utils;
mod tcp_middlebox;
mod mdns;

fn main(){
    utils::show_logo();

    loop {
        println!("1. MDSN protocol attack");
        println!("2. TCP middlebox reflection attack");
        println!("999. Exit");
        
        let option_number_str= utils::get_input_data().unwrap_or("999".to_string());
        let option_number: usize= option_number_str.parse().unwrap_or(0);
        match option_number {
            0 => continue,
            1 => tcp_middlebox::init(),
            2 => mdns::init(),
            999 => break,
            _ => continue
        }
    }
}

