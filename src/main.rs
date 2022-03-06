use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    address: Address,
    phone_numbers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Address {
    street: String,
    city: String,
    country: String,
}

fn main() {
    let mut file = File::open("text.json").unwrap();

    let person: Person = serde_json::from_str(file).expect("JSON was not well-formatted");
    println!("{:?}", person)
}
