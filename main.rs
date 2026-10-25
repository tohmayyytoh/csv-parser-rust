use std::io::{self, BufRead};

// TODO (csv-basics): implement per the lesson description.

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        let mut row: Vec<String> = Vec::new();
        let mut field: String = String::new();
        let char = l.chars();
        let delimiter = ',';
        for c in char {
            if c != delimiter {
                field.push(c);
            } else {
                row.push(field.clone());
                field.clear();
            }
        }

        row.push(field.clone());
        field.clear();
        println!("{:?}", row.join(" | "));
    }
}
