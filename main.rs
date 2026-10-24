use std::io::{self, BufRead};

// TODO (csv-basics): implement per the lesson description.

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        for c in l.chars() {
            for l in line {
                if c == ',' {
                    print!(" | ");
                } else {
                    print!("{c}");
                }
                println!("\n");
            }
        }
    }
}
