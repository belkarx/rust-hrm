pub mod utils {
    pub fn get_subject_info() {
        use std::io::{stdin,stdout,Write};
        let mut s = String::new();
        print!("Alias: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        println!("You typed: {}",s);
    }
}

