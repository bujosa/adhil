use adhil::vectors::vectors;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Municipio {
    name: String,
    province: String,
}

fn main() {
    // Example using my own module
    let x = vectors();

    println!("{}", x);

    // Open the file
    let mut file = File::open("data/municipios.json").expect("Failed to open file");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Print out the contents of the file
    println!("File contents: {}", contents);

    // Deserialize the JSON data into a Rust struct or value
    let data: Result<Vec<Municipio>, serde_json::Error> = serde_json::from_str(&contents);

    // Check for any errors during parsing
    match data {
        Ok(data) => {
            for obj in data {
                println!("{:?}", obj.name);
            }
        }
        Err(e) => {
            println!("error reading json: {}", e);
        }
    }
}
