use serde::Deserialize;
use std::fs;
use std::collections::BTreeMap as Map;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Subject {
    alias: String, //required
    uses: Vec<String>, //required
        occupation: String,
    source: String,
    contact: String,
    name: String, 
    age: Age,
}

#[derive(Debug, Deserialize)]
struct Address {
    street: String,
    city: String,
    country: String,
}

fn main() {
    let file = fs::read_to_string("text.json").expect("Unable to read file");

    let person: Person = serde_json::from_str(&file).expect("JSON was not well-formatted");
    println!("{:?}", person)
}

#[derive(Debug)]
enum Age {
    Old,
    Young,
    Uni,
    School
}
