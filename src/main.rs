//use std::collections::HashMap;
mod utils;
use utils::get_string;

fn main() {
    let alias = get_string("Alias: ");
    let contact_raw = get_string("Contact info (comma separated): ");
    let contacts:Vec<&str> = contact_raw.split(", ").collect();
    let contacts2:Vec<&str> = get_string("Contact info (comma separated): ").as_ref().split(", ").collect();

    as_ref()
    let uses = get_string("Uses (comma separated): ");
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
