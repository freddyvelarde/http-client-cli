//--url "https://www.google.com" --method "post" --body "body" --header "header"
use crate::data::HttpData;
use std::env;

fn checking_arguments(args: &Vec<String>) -> bool {
    let mut counter = 0;
    let valid_flags = vec![
        "--url", "-u", "--method", "-m", "--body", "-b", "--header", "-h",
    ];

    for arg in args {
        for flag in &valid_flags {
            if arg == flag {
                counter += 1;
            }
        }
    }
    return counter == valid_flags.len() / 2;
}

pub fn args() {
    let args: Vec<String> = env::args().collect();
    if !checking_arguments(&args) {
        return print!("--url or --method or --body or --header flag missing");
    }
    let mut http_data = HttpData {
        header: String::from(""),
        body: String::from(""),
        method: String::from(""),
        url: String::from(""),
    };
    let mut index = 0;
    for arg in &args {
        match arg.as_str() {
            "--url" | "-u" => {
                http_data.url = args.get(index + 1).unwrap().to_string();
                // println!("url: {:#?}", args.get(index + 1).unwrap());
            }
            "--method" | "-m" => {
                http_data.method = args.get(index + 1).unwrap().to_string();
                // println!("method: {:#?}", args.get(index + 1).unwrap());
            }
            "--body" | "-b" => {
                http_data.body = args.get(index + 1).unwrap().to_string();
                // println!("body: {:#?}", args.get(index + 1).unwrap());
            }
            "--header" | "-h" => {
                http_data.header = args.get(index + 1).unwrap().to_string();
                // println!("header: {:#?}", args.get(index + 1).unwrap());
            }
            _ => (),
        }
        index += 1;
    }
    http_data.print_data();
}
