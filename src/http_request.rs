use isahc::{prelude::*, Request};

pub struct HttpData {
    pub header: Vec<String>,
    pub body: String,
    pub url: String,
    pub method: String,
}

fn normalize_header(headers: &Vec<String>) -> Vec<Vec<String>> {
    let mut header_splited: Vec<Vec<String>> = vec![];
    for header in headers {
        let s: Vec<String> = header.split(':').map(String::from).collect();
        header_splited.push(s);
    }
    return header_splited;
}

impl HttpData {
    fn get_request(&self) -> Result<(), isahc::Error> {
        let mut response = isahc::get(&self.url)?;
        let body = response.text()?;
        println!("Body: {}", body);
        Ok(())
    }

    fn post_request(&self) -> Result<(), isahc::Error> {
        let mut request = Request::post(&self.url);

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
            "PATCH" | "patch" => Ok(println!("it's a PATCH request")),
            "DELETE" | "delete" => Ok(println!("it's a DELETE request")),
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
    // }
}
