use std::collections::HashMap;

pub struct Trie {
    prefix: String,
    children: HashMap<char, Trie>,
    is_terminal: bool,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            prefix: "".to_owned(),
            children: HashMap::new(),
            is_terminal: true,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = self;
        let chars: Vec<_> = word.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            curr = curr
                .children
                .entry(*c)
                .and_modify(|t| t.is_terminal |= i == chars.len() - 1)
                .or_insert(Trie {
                    prefix: word[..=i].to_owned(),
                    children: HashMap::new(),
                    is_terminal: i == chars.len() - 1,
                });
        }
    }

    pub fn find(&self, word: &str) -> Option<&Trie> {
        let mut curr = self;
        for c in word.chars() {
            match curr.children.get(&c) {
                None => return None,
                Some(t) => curr = t,
            };
        }

        Some(curr)
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}
