use isahc::prelude::*;

pub struct HttpData {
    pub header: String,
    pub body: String,
    pub url: String,
    pub method: String,
}

impl HttpData {
    fn get_request(&self) -> Result<(), isahc::Error> {
        let mut response = isahc::get(&self.url)?;
        // println!("Status: {}", response.status());
        let body = response.text()?;
        println!("Body: {}", body);
        Ok(())
    }

    pub fn http_request(&self) {
        let _ = match self.method.as_str() {
            "GET" | "get" => self.get_request(),
            "POST" | "post" => Ok(println!("it's a POST request")),
            "PUT" | "put" => Ok(println!("it's a PUT request")),
            "DELETE" | "delete" => Ok(println!("it's a DELETE request")),
            _ => Ok(()),
        };
    }

    // pub fn print_data(&self) {
    //     println!("url:  :) {}", self.url);
    //     println!("method: {}", self.method);
    //     println!("body: {}", self.body);
    //     println!("header: {}", self.header);
    // }
}
