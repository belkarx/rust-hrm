use std::collections::HashMap;
mod utils;
use crate::utils::utils::get_subject_info;

fn main() {
    utils::get_subject_info();
    //let mut book_reviews:HashMap<String, String> = HashMap::new();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
