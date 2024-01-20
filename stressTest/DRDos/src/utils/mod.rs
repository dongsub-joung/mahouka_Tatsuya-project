use std::{io::{stdin, BufRead, self, ErrorKind}, error};

pub fn show_logo(){
    println!("--------------------------------------------------------------");
    println!("d888888b  .d8b.  d888888b .d8888. db    db db    db  .d8b.");
    println!("`~~88~~' d8' `8b `~~88~~' 88'  YP 88    88 `8b  d8' d8' `8b ");  
    println!("   88    88ooo88    88    `8bo.   88    88  `8bd8'  88ooo88 ");
    println!("   88    88~~~88    88      `Y8b. 88    88    88    88~~~88 ");
    println!("   88    88   88    88    db   8D 88b  d88    88    88   88 ");
    println!("   YP    YP   YP    YP    `8888Y' ~Y8888P'    YP    YP   YP ");
    println!("--------------------------------------------------------------");
}

pub fn get_input_data()-> Result<String, std::io::Error>{
    let std= stdin();
    let mut buf= std.lock().lines();
    let option_number= buf.next().unwrap()?;
    let number= match option_number {
        n => return Ok(n),
        _=> return Err(io::Error::new(ErrorKind::InvalidData, "invalid data")),
    };
}