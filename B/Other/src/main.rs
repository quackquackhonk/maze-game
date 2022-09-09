use std::io;
use std::io::BufRead;
use std::process;


fn main() {
    let stdin = io::stdin();
    let mut res = String::from("\"");
    for line in stdin.lock().lines() {
        match line {
            Ok(line) if validate(&line)=> res.push(line.chars().nth(1).expect("Valid strings are always length 3")),
            _ => { 
                println!("unacceptable input");
                process::exit(1);
            },
        }
    }
    res.push('\"');
    println!("{res}");
}

fn validate(line: &str) -> bool {
    let valid_chars = vec!["\"┐\"", "\"└\"", "\"┌\"", "\"┘\""];
    valid_chars.contains(&line)
}

