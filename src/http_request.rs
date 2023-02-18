use crate::helpers::{normalize_header, print_in_json_format};
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
        println!("Status: {}", response.status());
        print_in_json_format(response.text()?);
        Ok(())
    }

    fn post_request(&self) -> Result<(), isahc::Error> {
        let mut request = Request::post(&self.url);

        for h in normalize_header(&self.header) {
            request = request.header(&h[0], &h[1]);
        }

        let mut response = request.body(format!(r#"{}"#, &self.body))?.send()?;

        println!("{}", response.status());
        // println!("{:#?}", response.text()?);
        print_in_json_format(response.text()?);

        Ok(())
    }

    fn delete_request(&self) -> Result<(), isahc::Error> {
        let mut response = isahc::delete("http://example.com/resource")?;

        println!("Response status: {}", response.status());
        println!("Response body: {}", response.text()?);

        Ok(())
    }

    fn patch_request(&self) -> Result<(), isahc::Error> {
        let mut request = Request::patch(&self.url);

        for h in normalize_header(&self.header) {
            request = request.header(&h[0], &h[1]);
        }

        let mut response = request.body(format!(r#"{}"#, &self.body))?.send()?;

        println!("{}", response.status());
        println!("{}", response.text()?);

        Ok(())
    }

    fn put_request(&self) -> Result<(), isahc::Error> {
        let mut request = Request::put(&self.url);

        for h in normalize_header(&self.header) {
            request = request.header(&h[0], &h[1]);
        }

        let mut response = request.body(format!(r#"{}"#, &self.body))?.send()?;

        println!("{}", response.status());
        println!("{}", response.text()?);

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
    // pub fn print_multi_header(&self) {
    //     let headers = normalize_header(&self.header);
    //     println!("{:#?}", &headers[0][0]);
    //     println!("{:#?}", &headers[0][1]);
    // }

    // pub fn print_data(&self) {
    //     println!("url:  :) {}", self.url);
    //     println!("method: {}", self.method);
    //     println!("body: {}", self.body);
    //     println!("header: {:#?}", self.header);
    //     println!("path: {:#?}", self.path);
    // }
}
