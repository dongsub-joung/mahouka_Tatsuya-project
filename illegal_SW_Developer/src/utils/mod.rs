use std::str::Bytes;

use rand::prelude::*;
use rand::Rng;
use hex;

pub fn generate_random_string() -> String{
    let mut result= String::new();
    let mut idx= 0;
    while idx < 1000 {
        let one_random_integer= generate_random();
        let c= one_random_integer.unwrap() as char;

        result.push_str(&format!("{}", c));
        
        idx+=1;
    }

    result
}

pub fn generate_random_hex_string(length: usize) -> String {
    let mut rng = rand::rng();
    let mut bytes = vec![0u8; length / 2]; // Each byte becomes two hex characters
    rng.fill(&mut bytes[..]);

    hex::encode(bytes)
}

pub fn ip_to_bytes (ip: String) -> Vec<u8>{
    let mut ip_bytes: Vec<Vec<u8>> = Vec::new();

    for oct in ip.split("."){
        let oct_int= oct.parse::<i32>().unwrap();
        let byte_oct= Vec::from(oct_int.to_be_bytes());
        ip_bytes.push(byte_oct);
        
        ip_bytes
    }

    ip_bytes
}

fn generate_random() -> Option<u8> {
    let mut rng = rand::rng();
    let mut nums: Vec<u8> = (33..126).collect();
    nums.shuffle(&mut rng);
    
    let result= *(nums.choose(&mut rng).unwrap());
    
    Some(result)
}
