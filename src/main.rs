//use std::collections::HashMap;
mod utils;

fn main() {
    let alias = utils::get_string("Alias: ");
    let contact_raw = utils::get_string("Contact info (comma separated): ");
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
