use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

pub struct Fb2Parser;

impl Fb2Parser {
    pub fn read_dir(dir_path: &str) -> Vec<String> {
        use std::fs;

        let paths = fs::read_dir(dir_path).unwrap();

        let files = paths
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| [dir_path, s].join("/").to_string()))
                })
            })
            .collect::<Vec<String>>();

        files
    }

    pub fn parse_file(file: &str) -> Result<Vec<String>> {
        let mut words: Vec<_> = Vec::new();
        let file = match File::open(file) {
            Ok(f) => f,
            Err(err) => return Err(err),
        };

        for line in BufReader::new(file).lines() {
            for word in line
                .unwrap()
                .rsplit_terminator(|c: char| !c.is_ascii_alphabetic())
            {
                match word.len() {
                    0 => continue,
                    _ => words.push(word.to_lowercase().clone()),
                }
            }
        }

        Ok(words)
    }
}
