use colored_json::ToColoredJson;
use serde_json::to_string_pretty;
use serde_json::{Result, Value};

pub fn normalize_header(headers: &Vec<String>) -> Vec<Vec<String>> {
    let mut header_splited: Vec<Vec<String>> = vec![];
    for header in headers {
        let s: Vec<String> = header.split(':').map(String::from).collect();
        header_splited.push(s);
    }
    return header_splited;
}

pub fn print_pretty_response(data: String, status: String) -> Result<()> {
    // const COLOR_RED: &str = "\x1b[31m";
    const COLOR_GREEN: &str = "\x1b[32m";
    const COLOR_RESET: &str = "\x1b[0m";
    const COLOR_YELLOW: &str = "\x1b[33m";

    let v: Value = serde_json::from_str(&format!(r#"{}"#, data.to_string()))?;

    let pretty_json = to_string_pretty(&v)?;

    println!(
        "{}{}{}",
        COLOR_GREEN, "---------------- STATUS ----------------", COLOR_RESET
    );
    println!("{}STATUS: {}{}", COLOR_YELLOW, status, COLOR_RESET);
    println!(
        "{}{}{}",
        COLOR_GREEN, "--------------- RESPONSE ---------------", COLOR_RESET
    );
    println!("{}", pretty_json.to_colored_json_auto()?);

    Ok(())
}
