//--url "https://www.google.com" --method "post" --body "body" --header "header"
use std::env;

fn checking_arguments(args: &Vec<String>) -> bool {
    let mut counter = 0;
    for arg in args {
        if arg == "--url" || arg == "--method" || arg == "--body" {
            counter += 1;
        }
    }
    return counter == 3;
}

pub fn args() {
    let args: Vec<String> = env::args().collect();
    if !checking_arguments(&args) {
        return print!("--url or --method or --body flag missing");
    }
    let mut index = 0;
    for arg in &args {
        match arg.as_str() {
            "--url" => {
                println!("url: {:#?}", args.get(index + 1).unwrap());
            }
            "--method" => {
                println!("method: {:#?}", args.get(index + 1).unwrap());
            }
            "--body" => {
                println!("body: {:#?}", args.get(index + 1).unwrap());
            }
            _ => (),
        }
        index += 1;
    }
}
