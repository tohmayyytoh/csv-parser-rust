use std::io::{self, BufRead};

#[derive(PartialEq)]
pub enum Variables {
    SetDelim(char),
    ParseRaw(String),
}

#[derive(Debug)]
struct CsvParser {
    delim: char,
}

impl CsvParser {
    fn process(&mut self, command: Variables) -> Option<Vec<String>> {
        let mut row = Vec::new();
        let mut field = String::new();
        match command {
            Variables::SetDelim(new_delim) => {
                self.delim = new_delim;
                None
            }
            Variables::ParseRaw(string) => {
                for c in string.chars() {
                    if c != self.delim {
                        field.push(c);
                    } else {
                        row.push(field.clone());
                        field.clear()
                    }
                }
                row.push(field.clone());
                field.clear();
                Some(row)
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut delimiter = CsvParser { delim: ',' };
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        let char = l.chars();

        if l.starts_with("DELIMITER ") {
            let (instruction, d) = l.split_at("DELIMITER ".len());
            let d = d.chars().next().unwrap();
            delimiter.process(Variables::SetDelim(d));
        }

        if l.starts_with("PARSE ") {
            let (instruction, p) = l.split_at("PARSE ".len());
            let p = p.to_string();
            let return_value = delimiter.process(Variables::ParseRaw(p));
            println!("OK\n{}", return_value.unwrap().join(" | "));
        }
    }
}
