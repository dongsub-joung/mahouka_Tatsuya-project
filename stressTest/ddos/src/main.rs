mod http_flood;
mod utils;

fn main(){
    utils::show_logo();
    let site= "example.com";
    let url= format!("http://{site}");
    let times= 2;
    let http_flood= http_flood::HttpFlood::new(url, times);

    http_flood.get_resource_attack();
    
    // limit 4k byte
    // http_flood.post_requeste_attack(utils::get_empty_text_data());
}