use std::collections::{HashMap, LinkedList};

pub struct Threegramindex {
    files: Vec<String>,
    index: HashMap<String, LinkedList<String>>,
}

impl Threegramindex {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files,
            index: std::collections::HashMap::new(),
        }
    }
}

impl Threegramindex {
    pub fn get_index(&self) -> &HashMap<String, LinkedList<String>> {
        &self.index
    }

    pub fn get_files(&self) -> Vec<String> {
        self.files.to_vec()
    }

    pub fn add_word(&mut self, word: String) {
        let w = format!("${}$", word);

        for i in 0..w.len() - 2 {
            let key = &w[i..i + 3];
            if self.index.contains_key(key) {
                let container = &mut self.index.get_mut(key).unwrap();
                container.push_back(word.to_owned());
            } else {
                &self
                    .index
                    .insert(key.to_string(), std::collections::LinkedList::new())
                    .unwrap()
                    .push_back(word.to_owned());
            }
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(1, 1);
}
