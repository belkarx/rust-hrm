use std::collections::HashMap;
mod utils;
use utils::get_string;
use serde::{Serialize, Deserialize};
use cursive::views::{Dialog, TextView, LinearLayout, SelectView};
use cursive::Cursive;
use cursive::traits::*;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    alias: String,
    name: Option<String>, //optional
    main_contact: String,
    other_contacts: Option<Vec<String>>, //optional
    uses: Vec<String>,
    skill: i16, //may be removed
    social: i16,
    source: String
}

fn init_person() -> Person {
    Person {
        alias: get_string("Alias: "),
        name: Some(get_string("Name [optional]: ")).filter(|s| s.len() > 0),
        main_contact: get_string("Main contact: "),
        other_contacts: {
            let other_contacts: Vec<String> = get_string("Other contacts (comma separated) [optional]: ").split(", ").map(|s| s.to_string()).collect();
            if other_contacts[0].len() > 0 {
                Some(other_contacts)
            } else { None }
            /*if !other_contacts.is_empty() {
                Some(other_contacts)
            } else {
                None
            }*/
        },
        uses: get_string("Uses (comma separated): ").split(", ").map(|s| s.to_string()).collect(),
        skill: get_string("Technical skill from 0 to 5: ").trim().parse::<i16>().expect("Enter a number lol"), //may be removed
        social: get_string("Social usefulness from 0 to 5: ").trim().parse::<i16>().expect("Enter a number lol"),
        source: get_string("Where did you find this person (IRL, Discord, Reddit, School, etc): ")
    }
}

fn write_to_file(p: Vec<Person>) {
    fs::write("data.json", serde_json::to_string(&p).unwrap()).unwrap();
}

fn read_from_file() -> Vec<Person> {
    let data: Hashmap<String,Person> = serde_json::from_str(&fs::read_to_string("data.json").unwrap()).unwrap();
    println!("{:#?}", data);
    data
}

fn main() {
    //let person = init_person();
    //println!("{:#?}", person);


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
