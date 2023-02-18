use crate::helpers::{normalize_header, print_pretty_response};
use isahc::{prelude::*, Request};

pub struct HttpData {
    pub header: Vec<String>,
    pub body: String,
    pub url: String,
    pub method: String,
    pub path: Vec<String>,
}

impl HttpData {
    fn get_request(&self) -> Result<(), isahc::Error> {
        let mut response = isahc::get(&self.url)?;

        print_pretty_response(response.text()?, response.status().to_string());

        Ok(())
    }

    fn post_request(&self) -> Result<(), isahc::Error> {
        let mut request = Request::post(&self.url);

        for h in normalize_header(&self.header) {
            request = request.header(&h[0], &h[1]);
        }

        let mut response = request.body(format!(r#"{}"#, &self.body))?.send()?;

        print_pretty_response(response.text()?, response.status().to_string());

        Ok(())
    }

    fn delete_request(&self) -> Result<(), isahc::Error> {
        let mut response = isahc::delete("http://example.com/resource")?;

        print_pretty_response(response.text()?, response.status().to_string());

        Ok(())
    }

    fn patch_request(&self) -> Result<(), isahc::Error> {
        let mut request = Request::patch(&self.url);

        for h in normalize_header(&self.header) {
            request = request.header(&h[0], &h[1]);
        }

        let mut response = request.body(format!(r#"{}"#, &self.body))?.send()?;

        print_pretty_response(response.text()?, response.status().to_string());

        Ok(())
    }

    fn put_request(&self) -> Result<(), isahc::Error> {
        let mut request = Request::put(&self.url);

        for h in normalize_header(&self.header) {
            request = request.header(&h[0], &h[1]);
        }

        let mut response = request.body(format!(r#"{}"#, &self.body))?.send()?;

        print_pretty_response(response.text()?, response.status().to_string());

        Ok(())
    }

    pub fn http_request(&self) {
        let _ = match self.method.as_str() {
            "GET" | "get" => self.get_request(),
            "POST" | "post" => self.post_request(),
            "PUT" | "put" => self.put_request(),
            "PATCH" | "patch" => self.patch_request(),
            "DELETE" | "delete" => self.delete_request(),
            _ => Ok(()),
        };
    }
}
