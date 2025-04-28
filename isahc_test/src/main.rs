use isahc::prelude::*;

fn main(){
    let mut response = isahc::get_async("https://google.com");
    println!("{:#?}", response);
}