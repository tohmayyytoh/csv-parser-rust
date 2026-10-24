use std::io::{self, BufRead};

// csv files are of the type ->
// name,age,city
// Alice,30,NYC
// Bob,25,LA
// ,empty,first

fn main() {
    let stdin = io::stdin();
    'first: for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        let character = l.chars();
        if l == "DELIMITER ;" {
            print!("OK");
            println!();
            continue 'first;
        } else if l == "PARSE" {
            continue;
        }
        for c in character {
            if c == ',' || c == ';' || c.is_whitespace() {
                print!(" | ");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}
