use serde::Deserialize;
use std::fs;
use std::collections::BTreeMap as Map;
use std::error::Error;

#[derive(Debug, Deserialize, Default)]
struct Subject {
    alias: String, //required
    uses: Uses, //required
    source: String,
    contact: String,
    name: String, 
}


impl<T> Default for A<T> {
    fn default() -> Self {
        Self { field: Default::default() }
    }
}

#[derive(Debug, Deserialize, Default)]
struct Uses {
    form: String,
    details: Vec<String>,
}

fn main() {
    let file = fs::read_to_string("text.json").expect("Unable to read file");

    let person: Person = serde_json::from_str(&file).expect("JSON was not well-formatted");
    println!("{:?}", person)
}
/*
enum Form {
    Professional,
    Informative,
    Social
}*/
