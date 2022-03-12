//use std::collections::HashMap;
mod utils;
use utils::get_string;

fn main() {
    let alias = get_string("Alias: ");
    let name = get_string("Name [optional]: ");

    let main_contact = get_string("Main contact: ");
    let other_contacts_raw = get_string("Other contacts (comma separated) [optional]: ");
    let other_contacts:Vec<&str> = other_contacts_raw.split(", ").collect();

    let uses_raw = get_string("Uses (comma separated): ");
    let uses:Vec<&str> = uses_raw.split(", ").collect();

    let skill = get_string("Technical skill from 0 to 5: ").trim().parse::<i16>().unwrap();
    let social = get_string("Social usefulness from 0 to 5: ").trim().parse::<i16>().unwrap();
    
    println!("{}", alias);
    if !name.is_empty() {
        println!("{}", name);
    }
    println!("{:#?}",contacts);
    if !other_contacs.is_empty() {
        println!("{}", name);
    }
    println!("{:#?}",uses);
    println!("{}", skill);
    println!("{}", social);
    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
