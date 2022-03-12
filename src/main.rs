//use std::collections::HashMap;
mod utils;
mod ui;
use ui::tui;
use utils::get_string;
use serde::{Serialize, Deserialize};

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

fn main() {
    let person = init_person();
    println!("{:#?}", person);
    tui().unwrap();
    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
