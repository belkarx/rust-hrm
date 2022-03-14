use std::collections::HashMap;
mod utils;
use utils::get_string;
use serde::{Serialize, Deserialize};
use std::fs;

//TODO: Tui and add proper get_string prompts

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
        name: Some(get_string("Name [optional]: "))
            .filter(|s| s.len() > 0),
        main_contact: get_string("Main contact: "),
        other_contacts: {
            let other_contacts: Vec<String> = get_string("Other contacts (comma separated) [optional]: ")
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            if other_contacts[0].len() > 0 {
                Some(other_contacts)
            } else { None }
        },
        uses: get_string("Uses (comma separated): ")
            .split(", ")
            .map(|s| s.to_string())
            .collect(),
        skill: get_string("Technical skill from 0 to 5: ")
            .trim()
            .parse::<i16>()
            .expect("Enter a number lol"), //may be removed
        social: get_string("Social usefulness from 0 to 5: ")
            .trim()
            .parse::<i16>()
            .expect("Enter a number lol"),
        source: get_string("Where did you find this person (IRL, Discord, Reddit, School, etc): ")
    }
}

impl Person {
    fn set_name(&mut self) {
        self.name = Some(get_string("Name [optional]: "))
            .filter(|s| s.len() > 0);
    }

    fn add_other_contacts(&mut self) {
        let mut added_contacts: Vec<String> = get_string("Contacts to add: ")
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        
        if self.other_contacts != None {
            if let Some(ref mut x) = self.other_contacts {
                x.append(&mut added_contacts);
            }
        } else {
            self.other_contacts = Some(added_contacts)
        }
    }

    fn add_uses(&mut self) {
        let mut added: Vec<String> = get_string("uses to add: ")
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        self.uses.append(&mut added);
    }

    fn del_uses(&mut self) {
        println!("{:#?}", self.uses);
        for (i, u) in self.uses.iter().enumerate() {
            println!("{} | {}", i+1, u);
        }
        println!();
        let idx = get_string("Which use would you like to delete: ")
            .trim()
            .parse::<usize>()
            .expect("Enter a number lol")-1;
        self.uses.remove(idx);
    }
}

fn print_keys(hm: &HashMap<String, Person>) {
    for key in hm.keys() {
        println!("{}", key);
    };
    println!();
}

fn main() {
    let mut hm: HashMap<String, Person> = serde_json::from_str(&fs::read_to_string("data.json").unwrap()).unwrap();
    let mut modified: bool = false;

    loop {
        let choice = get_string("------------------------------------------------------------------------------------\nMENU: [C]reate [R]ead [U]pdate [D]elete == Sort : [S]source [T]echnicality == [Q]uit\n-------------------------------------------------------------------------------------\n",
        ).to_lowercase();
        match choice.as_str() {
            "r" => {
                print_keys(&hm);
                let key = get_string("Which would you like to read (q if none): ");
                if key != "q" {
                    println!("{:#?}", hm.get_mut(&key).unwrap());
                }
            },
            "q" => {
                if modified {
                    fs::write("data.json", serde_json::to_string(&hm).unwrap()).unwrap();
                }
                break;
            },
            "c" => {
                let person = init_person();
                println!("{:#?}", person);
                hm.insert(person.alias.clone(), person);
                println!("Person added successfully");
                modified = true;
            },
            "d" => {
                print_keys(&hm);
                let key = get_string("Which would you like to delete: ");
                hm.remove(&key);
                modified = true;
            },
            "u" => {
                print_keys(&hm);
                let key = get_string("Which would you like to alter: ");
                let p = hm.get_mut(&key).unwrap();
                println!("{:#?}", p);
                println!("\nYou can alter\n    - name\n    - other_contacts\n    - uses\n");
                
                match get_string("choose a field to alter: ").as_str() {
                    "name" => p.set_name(),
                    "other_contacts" => p.add_other_contacts(),
                    "uses" => {
                        if get_string("\nWould you like to add or delete uses: ").contains("a") {
                            p.add_uses()
                        } else { p.del_uses() }
                    },
                    _ => panic!("Not an option")
                } 

            },
            _ => panic!("choice not available")
        }
    }
}

/*
  let chosen = by_alias(&hm);
    let p = hm.get_mut(&chosen).unwrap();
    println!("{:#?}", p);
    println!("\nYou can alter\n    - name\n    - other_contacts\n    - uses\n");
    
    match get_string("choose a field to alter: ").as_str() {
        "name" => p.set_name(),
        "other_contacts" => p.add_other_contacts(),
        "uses" => {
            if get_string("\nWould you like to add or delete uses: ").contains("a") {
                p.add_uses()
            } else { p.del_uses() }
        },
        _ => panic!("Not an option")
    } 


    println!("{:#?}", p);
    

    //hm.remove(&chosen);

    /*loop {
        let person = init_person();
        println!("{:#?}", person);
        hm.insert(person.alias.clone(), person);
        if get_string("another? ").contains("n") {break;}
    };
    */

    read_from_file_as_hashmap();


 */


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
