pub fn normalize_header(headers: &Vec<String>) -> Vec<Vec<String>> {
    let mut header_splited: Vec<Vec<String>> = vec![];
    for header in headers {
        let s: Vec<String> = header.split(':').map(String::from).collect();
        header_splited.push(s);
    }
    return header_splited;
}
