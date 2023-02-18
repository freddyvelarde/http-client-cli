use serde_json::{json, to_string_pretty};
use serde_json::{Result, Value};

pub fn normalize_header(headers: &Vec<String>) -> Vec<Vec<String>> {
    let mut header_splited: Vec<Vec<String>> = vec![];
    for header in headers {
        let s: Vec<String> = header.split(':').map(String::from).collect();
        header_splited.push(s);
    }
    return header_splited;
}

pub fn print_in_json_format(data: String) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&format!(r#"{}"#, data.to_string()))?;

    // Access parts of the data by indexing with square brackets.
    let pretty_json = to_string_pretty(&v).unwrap();
    println!("{}", pretty_json);

    Ok(())
}

// pub fn print_pretty_json() {
//     let value = json!({
//         "name": "Alice",
//         "age": 25,
//         "pets": ["dog", "cat"],
//         "address": {
//             "street": "123 Main St",
//             "city": "Anytown",
//             "state": "CA",
//             "zip": "12345"
//         }
//     });
//
//     let pretty_json = to_string_pretty(&value).unwrap();
//     println!("{}", pretty_json);
// }
