//use std::collections::HashMap;
mod utils;

fn main() {
    let alias = utils::get_string("Alias: ");
    let contact = utils::get_string("Contact info (comma separated): ").split(", ").collect();
    let contacts: Vec<&str> = split.collect();
    println!("{}", alias);
    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
