use std::io::{self, BufRead};

// csv files are of the type ->
// name,age,city
// Alice,30,NYC
// Bob,25,LA
// ,empty,first

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        let character = l.chars();
        for c in character {
            if c == ',' {
                print!(" | ");
            } else {
                print!("{c}");
            }
        }
        print!("\n");
    }
}
