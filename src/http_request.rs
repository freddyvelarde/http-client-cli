use isahc::{prelude::*, Request};

pub struct HttpData {
    pub header: Vec<String>,
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

    fn post_request(&self) -> Result<(), isahc::Error> {
        let mut response = Request::post(&self.url)
            .header("Content-Type", "application/json")
            .body(format!(r#"{}"#, &self.body))?
            .send()?;

        // println!("{}", response.status());
        println!("{}", response.text()?);

        Ok(())
    }

    pub fn http_request(&self) {
        let _ = match self.method.as_str() {
            "GET" | "get" => self.get_request(),
            "POST" | "post" => self.post_request(),
            "PUT" | "put" => Ok(println!("it's a PUT request")),
            "DELETE" | "delete" => Ok(println!("it's a DELETE request")),
            _ => Ok(()),
        };
    }

    // pub fn print_data(&self) {
    //     println!("url:  :) {}", self.url);
    //     println!("method: {}", self.method);
    //     println!("body: {}", self.body);
    //     println!("header: {:#?}", self.header);
    // }
}
