//--url "https://www.google.com" --method "post" --body "body" --header "header"
use std::env;

pub fn args() {
    let args: Vec<String> = env::args().collect();
    let mut counter = 0;
    for arg in &args {
        // if arg == "--url" {
        //     println!("{:#?}", args.get(counter + 1).unwrap());
        // } else {
        //     println!("there is no --url argument passed.");
        //     return;
        // }
        match arg.as_str() {
            "--url" => {
                println!("url: {:#?}", args.get(counter + 1).unwrap());
            }
            "--method" => {
                println!("method: {:#?}", args.get(counter + 1).unwrap());
            }
            "--body" => {
                println!("body: {:#?}", args.get(counter + 1).unwrap());
            }
            _ => (),
        }
        counter += 1;
    }
}
