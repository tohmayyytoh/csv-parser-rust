/*use std::io::{self, BufRead};

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
}*/

use std::io::{self, BufRead};

// TODO (csv-basics): implement per the lesson description.

fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        lines.push(l);
        /*for c in l.chars() {
            if c == ',' {
                print!(" | ");
            } else {
                print!("{c}");
            }
            println!("\n");
        }*/
    }
    for l in lines {
        for c in l.chars() {
            if c == ',' {
                print!(" | ");
            } else {
                print!("{c}");
            }
            print!("\n");
        }
    }
}
