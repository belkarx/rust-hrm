//use std::collections::HashMap;
mod utils;
use utils::get_string;

fn main() {
    let alias = get_string("Alias: ");
    
    let contact_raw = get_string("Contact info (comma separated): ");
    let contacts:Vec<&str> = contact_raw.split(", ").collect();

    let uses_raw = get_string("Uses (comma separated): ");
    let contacts:Vec<&str> = contact_raw.split(", ").collect();
    
    println!("{}", alias);
    println!("{:#?}",contacts);
    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
