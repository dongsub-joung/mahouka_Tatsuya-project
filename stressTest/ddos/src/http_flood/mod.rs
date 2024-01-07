// 1. post request in the form
// 2. get resource(html,css,js)
pub struct HttpFlood{
    target_url: String,
    times: u32
}

impl HttpFlood {
    pub fn new(target_url: String, times: u32) -> Self{
        Self{
            target_url, times
        }
    }

    pub fn post_requeste_attack(&self, text: String){

    }

    pub fn get_resource_attack(&self){

    }
}

