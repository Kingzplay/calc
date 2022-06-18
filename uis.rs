use std::io::{self,Write};

pub fn uis(x: String) -> String {
    print!("{}",x);
    let mut prompt = String::new();
    io::stdout().flush();
    io::stdin().read_line(&mut prompt).expect("Failed to read line");
    let prompt = prompt.trim();
    return prompt.to_string();
}