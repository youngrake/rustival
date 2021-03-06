use std::fs::File;
use std::io::BufRead;

struct Node {
    children: std::collections::HashMap<char, Node>,
    files: Vec<String>,
    character: char,
}

impl Node {
    pub fn new(character: char) -> Self {
        Self {
            character,
            children: std::collections::HashMap::new(),
            files: Vec::new(),
        }
    }

    fn add_child(&mut self, letter: char) -> Option<Node> {
        self.children.insert(letter, Node::new(letter))
    }
}

pub struct ReversedTree {
    nodes: std::collections::HashMap<char, Node>,
}

impl ReversedTree {
    pub fn new() -> Self {
        Self {
            nodes: std::collections::HashMap::new(),
        }
    }
}

impl ReversedTree {
    pub fn process_word(word: &str) -> String {
        word.chars().rev().collect::<String>()
    }

    pub fn process_files(&mut self, files: &Vec<String>) {
        for file_name in files {
            let file = File::open(&file_name).expect("File Not Found");
            for line in std::io::BufReader::new(file).lines() {
                for word in line
                    .unwrap()
                    .split_terminator(|c: char| !c.is_ascii_alphabetic())
                {
                    if word.len() > 0 {
                        let word = ReversedTree::process_word(&word);
                        self.add_word(&word, &file_name);
                    }
                }
            }
        }
    }

    pub fn add_word(&mut self, word: &str, file_name: &str) {
        let first_letter = word.chars().nth(0).unwrap();
        if !&self.nodes.contains_key(&first_letter) {
            self.nodes.insert(first_letter, Node::new(first_letter));
        }

        let mut current_node = self.nodes.get_mut(&first_letter).unwrap();
        for i in 1..word.len() {
            let letter = word.chars().nth(i).unwrap();
            if !&current_node.children.contains_key(&letter) {
                &current_node.add_child(letter);
            }
            current_node = current_node.children.get_mut(&letter).unwrap();
        }

        if !&current_node.files.contains(&file_name.to_string()) {
            &current_node.files.push(file_name.to_string());
        }
    }

    pub fn find_word(&mut self, word: &str) -> Option<Vec<String>> {
        let word = ReversedTree::process_word(word);
        let first_letter = word.chars().nth(0).unwrap();
        if !&self.nodes.contains_key(&first_letter) {
            self.nodes.insert(first_letter, Node::new(first_letter));
        }

        let mut current_node = self.nodes.get_mut(&first_letter).unwrap();
        for i in 1..word.len() {
            let letter = word.chars().nth(i).unwrap();
            if !current_node.children.contains_key(&letter) {
                return None;
            }
            current_node = current_node.children.get_mut(&letter).unwrap();
        }

        Some(current_node.files.to_owned())
    }
}
