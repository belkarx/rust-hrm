//use std::collections::HashMap;
mod utils;
use utils::get_string;

struct Person {
    alias: String,
    name: Option<String>, //optional
    main_contact: String,
    other_contacts: Option<Vec<&'static str>>, //optional
    uses: Vec<&'static str>,
    skill: i16, //may be removed
    social: i16,
    source: String
}

fn init_person() {
    Person {
        alias: get_string("Alias: "),
        name: Some(get_string("Name [optional]: ")).filter(|n| n.len() > 0),
        main_contact: get_string("Main contact: "),
        other_contacts: {
            let other_contacts_raw: String = &get_string("Other contacts (comma separated) [optional]: ");
            let other_contacts:Vec<&str> = other_contacts_raw.split(", ").collect();
            if !other_contacts.is_empty() {
                Some(other_contacts)
            } else {
                None
            }
        },
        uses: {
            let uses_raw = get_string("Uses (comma separated): ");
            let uses:Vec<&str> = uses_raw.split(", ").collect();
            uses
        },
        skill: get_string("Technical skill from 0 to 5: ").trim().parse::<i16>().unwrap(), //may be removed
        social: get_string("Social usefulness from 0 to 5: ").trim().parse::<i16>().unwrap(),
        source: get_string("Where did you find this person (IRL, Discord, Reddit, School, etc): ")
    };
}
fn main() {

    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
