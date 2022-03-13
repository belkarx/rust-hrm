use std::collections::HashMap;
mod utils;
use utils::get_string;
use serde::{Serialize, Deserialize};
//use cursive::views::{Dialog, TextView, LinearLayout, SelectView};
//use cursive::Cursive;
//use cursive::traits::*;
use std::fs;
use serde_json::Map;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Person {
    fn set_alias(&mut self) {
        self.alias = get_string("Alias: ");
    }
    fn set_name(&mut self) {
        self.name = Some(get_string("Name [optional]: ")).filter(|s| s.len() > 0);
    }
    fn set_main_contact(&mut self) {
        self.main_contact = get_string("Main contact: ");
    }
    fn add_other_contacts(&mut self) {
        let mut added_contacts: Vec<String> = get_string("Contacts to add: ").split(", ").map(|s| s.to_string()).collect();
        self.other_contacts.unwrap_or(added_contacts.clone()).append(&mut added_contacts);
    }
    fn set_uses(&mut self) {
        self.uses = get_string("Uses (comma separated): ").split(", ").map(|s| s.to_string()).collect();
    }
    fn set_skill(&mut self) {
        self.skill = get_string("Technical skill from 0 to 5: ").trim().parse::<i16>().expect("Enter a number lol");
    }
    fn set_social(&mut self) {
        self.social = get_string("Social usefulness from 0 to 5: ").trim().parse::<i16>().expect("Enter a number lol");
    }
    fn set_source(&mut self) {
        self.source = get_string("Where did you find this person (IRL, Discord, Reddit, School, etc): ");
    }
}

/*fn write_to_file(p: Vec<Person>) {
    fs::write("data.json", serde_json::to_string(&p).unwrap()).unwrap();
}*/

fn write_to_file_as_hashmap(p: HashMap<String, Person>) {
    fs::write("data.json", serde_json::to_string(&p).unwrap()).unwrap();
}
/*
fn read_from_file() -> Vec<Person> {
    let data: Vec<Person> = serde_json::from_str(&fs::read_to_string("data.json").unwrap()).unwrap();
    println!("{:#?}", data);
    data
}
*/
fn read_from_file_as_hashmap() -> HashMap<String, Person> {
    let data: HashMap<String, Person> = serde_json::from_str(&fs::read_to_string("data.json").unwrap()).unwrap();
    println!("{:#?}", data);
    data
}

fn by_alias(hm: &HashMap<String, Person>) -> String {
    for key in hm.keys() {
        println!("{}", key);
    };
    println!();
    get_string("choose an alias to alter: ")
}
/*
fn by_field(mut p: &Person) {
    println!("{:#?}", p);
    let field = get_string("choose a field to alter: "); 
    let value = get_string("what value are you changing it to: ");
    match &field[..] {
        "alias" => {p.alias = value},
        
    };
}*/

fn main() {
    //let mut vec = read_from_file();
    let mut hm: HashMap<String, Person> = read_from_file_as_hashmap();
    let chosen = by_alias(&hm);
    let p = hm.get_mut(&chosen).unwrap();
    println!("{:#?}", p);
    let _field = get_string("choose a (TEXT) field to alter: "); 
    let _value = get_string("what value are you changing it to: ");
    /*    match &field[..] {
            //"alias" => p.alias = value, //alters hashmap key
            "name" => p.name = value,
            "" => p.name = value,
            _ => panic!("Lol field doesn't exist /shrug")
        };
*/

    //hm.remove(&chosen);

    /*loop {
        let person = init_person();
        println!("{:#?}", person);
        hm.insert(person.alias.clone(), person);
        if get_string("another? ").contains("n") {break;}
    };
    */

    write_to_file_as_hashmap(hm);
    read_from_file_as_hashmap();

}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
