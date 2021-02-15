use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use substring::Substring;

pub struct PermutationIndex {
    index: HashMap<String, HashSet<String>>,
    words: HashMap<String, HashSet<String>>,
}

impl PermutationIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            words: HashMap::new(),
        }
    }
}

impl PermutationIndex {
    pub fn process_word(word: &str) -> String {
        word.to_lowercase().chars().collect::<String>()
    }

    fn add_word(&mut self, word: String, file_name: &String) {
        let ref word = PermutationIndex::process_word(&word);

        match self.index.get_mut(word) {
            Some(found) => {
                found.insert(file_name.to_owned());
            }
            None => {
                self.index.insert(word.to_owned(), HashSet::new());
                let new_word = self.index.get_mut(word).unwrap();
                new_word.insert(file_name.to_owned());
            }
        }
        let mut set = HashSet::new();
        set.insert(format!("${}", word));
        for i in 0..word.len() {
            set.insert(format!(
                "{}${}",
                word.substring(i, word.len()),
                word.substring(0, i)
            ));
        }

        for key in &set {
            if self.words.contains_key(key) {
                let word_terms = self.words.get_mut(key).unwrap();
                word_terms.insert(word.to_owned());
            } else {
                self.words.insert(key.to_owned(), HashSet::new());
                let new_word = self.words.get_mut(key).unwrap();
                new_word.insert(word.to_owned());
            }
        }
    }

    pub fn process_files(&mut self, files: &Vec<String>) {
        for file_name in files {
            let file = File::open(&file_name).expect("File not Found");
            for line in std::io::BufReader::new(file).lines() {
                for word in line
                    .unwrap()
                    .split_terminator(|c: char| !c.is_ascii_alphabetic())
                {
                    if word.len() > 0 {
                        self.add_word(word.to_owned(), &file_name);
                    }
                }
            }
        }
    }

    pub fn search(&mut self, request: String) -> Option<HashSet<&String>> {
        let request = request.to_lowercase();
        let counter = request.matches("*").count();
        let mut matching_words: HashSet<&String> = HashSet::new();
        if counter == 0 {
            for key in self.words.keys().to_owned() {
                if key.contains(&format!("{}$", request)) || key.contains(&format!("${}", request))
                {
                    let words = self.words.get(key).unwrap();
                    matching_words.extend(words.iter());
                }
            }

            return Some(matching_words.to_owned());
        }
        if counter == 1 {
            let mut query = "".to_string();

            for i in 0..request.len() {
                if request.chars().nth(i).unwrap() != '*' {
                    let q = &format!("{}{}", query, request.chars().nth(i).unwrap());
                    query = q.to_string();
                } else {
                    query = format!("{}${}", request.substring(i + 1, request.len()), query);
                    break;
                }
            }
            for key in self.words.keys().to_owned() {
                if key.starts_with(&query) {
                    let words = self.words.get(key).unwrap();
                    matching_words.extend(words.iter());
                }
            }
            return Some(matching_words.to_owned());
        }
        if counter >= 2 {
            let f = match request.find("*") {
                Some(found) => found,
                None => return None,
            };
            let q1 = request.substring(0, f);

            let inner: Vec<_> = request.split("*").collect();
            let q2 = inner.last().copied().unwrap();

            let inner = inner[1..inner.len() - 1].to_vec();

            let q = &format!("{}${}", q2, q1);

            for key in self.words.keys().to_owned() {
                if key.starts_with(q) {
                    let words = self.words.get(key).unwrap();
                    for j in words {
                        let mut should_add = true;
                        let mut last: i32 = -1;
                        for c in &inner {
                            let subs = j.substring((last + 2) as usize, j.len());
                            if !j.contains(c) || subs.find(c) == None {
                                should_add = false;
                                break;
                            }
                            last = subs
                                .find(j.substring((last + 2) as usize, j.len()))
                                .unwrap() as i32;
                        }
                        if should_add {
                            matching_words.extend(words);
                        }
                    }
                }
            }
            println!("{:?}", matching_words);

            return Some(matching_words.to_owned());
        }
        None
    }
}
