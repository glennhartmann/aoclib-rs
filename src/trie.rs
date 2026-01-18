use std::collections::HashMap;

/// Recursively-defined prefix trie. Typically constructed with `Trie::new()` for the root node,
/// then `insert()`ed into many times. Querying for simple existence of a word in the `Trie` can be
/// performed with `contains()`.
///
/// More advanced usage includes using `find()` to find sub-`Trie` structures to efficiently search
/// for existince of many words with similar prefixes. See
/// <https://github.com/glennhartmann/aoc24/blob/fa21b5787382765a598381c3a0583258b645dc86/src/days/day_19.rs>
/// for an example.
///
/// Empty strings are considered to exist in every `Trie` by default.
pub struct Trie {
    prefix: String,
    children: HashMap<char, Self>,
    is_terminal: bool,
}

impl Trie {
    /// Create a new `Trie` root node containing the empty string.
    pub fn new() -> Self {
        Self::with_prefix("", true /* is_terminal */)
    }

    /// Create a sub-node with an existing prefix, whether terminal or not, or can be used to
    /// create a root node which does not contain the empty string.
    pub fn with_prefix(s: &str, is_terminal: bool) -> Self {
        Self {
            prefix: s.to_owned(),
            children: HashMap::new(),
            is_terminal,
        }
    }

    /// Insert a word into the `Trie`.
    pub fn insert(&mut self, word: &str) {
        let mut curr = self;
        let chars: Vec<_> = word.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            let is_terminal = i == chars.len() - 1;
            curr = curr
                .children
                .entry(*c)
                .and_modify(|t| t.is_terminal |= is_terminal)
                .or_insert(Self::with_prefix(&word[..=i], is_terminal));
        }
    }

    /// Find the node within the `Trie` described by `word`. Note that the returned node may not be
    /// terminal. Can be used as any other `Trie`, for example to continue searching for many
    /// suffixes that share the same prefix.
    pub fn find(&self, word: &str) -> Option<&Self> {
        let mut curr = self;
        for c in word.chars() {
            match curr.children.get(&c) {
                None => return None,
                Some(t) => curr = t,
            };
        }

        Some(curr)
    }

    /// Whether the `Trie` contains `word`.
    pub fn contains(&self, word: &str) -> bool {
        let Some(node) = self.find(word) else {
            return false;
        };

        node.is_terminal()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let t = Trie::new();
        assert!(t.contains(""));
        assert!(!t.contains("a"));
    }

    #[test]
    fn test_typical() {
        let mut t = Trie::new();
        assert!(t.contains(""));

        t.insert("asdf");
        t.insert("1234");
        t.insert("asteroid");
        t.insert("astronaut");
        t.insert("1235");

        assert!(t.contains(""));
        assert!(t.contains("asdf"));
        assert!(t.contains("1234"));
        assert!(t.contains("asteroid"));
        assert!(t.contains("astronaut"));
        assert!(t.contains("1235"));

        assert!(!t.contains("as"));
        t.insert("as");
        assert!(t.contains("as"));
        assert!(t.contains("asteroid"));
        assert!(t.contains("astronaut"));
        assert!(t.contains("asdf"));
    }
}
