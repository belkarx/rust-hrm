pub fn get_string(prompt: &str) -> String {
    use std::io::{stdin,stdout,Write};
    let mut s = String::new();
    print!(prompt);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}
