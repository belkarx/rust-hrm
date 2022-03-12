//use std::collections::HashMap;
mod utils;

fn main() {
    let x = utils::get_string("alias: ");
    println!("{}", x);
    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
