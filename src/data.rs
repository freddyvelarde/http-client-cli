pub struct HttpData {
    pub header: String,
    pub body: String,
    pub url: String,
    pub method: String,
}

impl HttpData {
    pub fn print_data(&self) {
        println!("url:  :) {}", self.url);
        println!("method: {}", self.method);
        println!("body: {}", self.body);
        println!("header: {}", self.header);
    }
}
