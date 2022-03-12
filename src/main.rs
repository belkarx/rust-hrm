//use std::collections::HashMap;
mod utils;
use utils::get_string;

fn main() {
    let alias = get_string("Alias: ");
    let name = get_string("Name: ");
    
    let contact_raw = get_string("Contact info (comma separated): ");
    let contacts:Vec<&str> = contact_raw.split(", ").collect();

    let uses_raw = get_string("Uses (comma separated): ");
    let uses:Vec<&str> = uses_raw.split(", ").collect();

    let skill = get_string("Technical skill from 0 to 10").trim().parse::<i64>().unwrap();
    
    println!("{}", alias);
    if !name.is_empty() {
        println!("{}", name);
    }
    println!("{:#?}",contacts);
    println!("{:#?}",uses);
    println!("{}", skill);
    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
