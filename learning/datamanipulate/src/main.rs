use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

fn read_json_file(filename: &str) -> Result<Person, std::io::Error> {
    let mut file = File::open(filename).expect("Couldn't open JSON file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Couldn't read file content.");

    let p: Person = serde_json::from_str(&content).expect("Couldn't parse JSON into Person struct.");

    Ok(p)
}

fn main() {
    let filename = "sample.json";

    match read_json_file(filename) {
        Ok(person) => {
            println!("{:?}", person);
        }
        Err(e) => {
            eprintln!("Error reading JSON file: {}", e);
        }
    }
}