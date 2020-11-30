use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}
impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            end: false,
        }
    }

    fn add(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(Trie::new());
        }
        node.end = true;
    }

    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return self.end;
        }
        let c = word.chars().next().unwrap();
        if c == '.' {
            for child in self.children.values() {
                if Self::search(&child, word[1..].to_string()) {
                    return true;
                }
            }
        } else {
            if let Some(child) = self.children.get(&c) {
                return Self::search(&child, word[1..].to_string());
            } else {
                return false;
            }
        }
        false
    }
}

struct WordDictionary {
    dictionary: Trie,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            dictionary: Trie::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        self.dictionary.add(word)
    }

    fn search(&self, word: String) -> bool {
        self.dictionary.search(word)
    }
}

#[test]
fn test() {
    let mut wd = WordDictionary::new();
    wd.add_word("bad".to_string());
    wd.add_word("dad".to_string());
    wd.add_word("mad".to_string());
    assert_eq!(wd.search("pad".to_string()), false);
    assert_eq!(wd.search("bad".to_string()), true);
    assert_eq!(wd.search(".ad".to_string()), true);
    assert_eq!(wd.search("b..".to_string()), true);
}
