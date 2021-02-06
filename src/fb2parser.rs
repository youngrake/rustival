use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

pub struct Fb2Parser;

impl Fb2Parser {
    pub fn parse_file(file: String) -> Result<Vec<String>> {
        let mut words: Vec<String> = Vec::new();
        let file = File::open(&file).expect("File not found");

        for line in BufReader::new(file).lines() {
            for word in line
                .unwrap()
                .split_terminator(|c: char| c.is_ascii_punctuation())
            {
                match word.len() {
                    0 => continue,
                    _ => words.push(word.to_string()),
                }
            }
        }

        Ok(words)
    }
}
