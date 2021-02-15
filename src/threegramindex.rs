use std::collections::HashMap;

pub struct Threegramindex {
    index: HashMap<String, Vec<String>>,
}

impl Threegramindex {
    pub fn new() -> Self {
        Self {
            index: std::collections::HashMap::new(),
        }
    }
}

impl Threegramindex {
    pub fn get_index(&self) -> &HashMap<String, Vec<String>> {
        &self.index
    }

    pub fn add_words(&mut self, words: &Vec<String>) {
        for word in words {
            self.add_word(word.to_string());
        }
    }

    pub fn add_word(&mut self, word: String) {
        let w = format!("${}$", word);

        for i in 0..w.len() - 2 {
            let key = &w[i..i + 3];
            if self.index.contains_key(key) {
                let container = &mut self.index.get_mut(key).unwrap();
                container.push(word.to_string());
            } else {
                &self.index.insert(key.to_string(), Vec::new());

                &self
                    .index
                    .get_mut(&key.to_string())
                    .unwrap()
                    .push(word.to_string());
            }
        }
    }
}
