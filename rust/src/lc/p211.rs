use std::collections::HashMap;

/// #Default, HashMap
struct WordDictionary {
    root: Trie
}

#[derive(PartialEq, Eq, Default, Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool
}


impl Trie {

    fn insert(&mut self, str: &str) {
        let mut p = self;
        for ch in str.chars() {
            p = p.children.entry(ch).or_default()
        }
        p.end = true;
    }

    fn search(&self, str: &str) -> bool {
        if str.is_empty() {
            return self.end;
        }
        let ch = str.chars().next().unwrap();
        if ch == '.' {
            for child in self.children.values() {
                if Self::search(child,&str[1..]) {
                    return true;
                }
            }
        } else {
            if let Some(t) = self.children.get(&ch) {
                return t.search(&str[1..]);
            } else {
                return false;
            }
        }
        false
    }
}

impl WordDictionary {

    fn new() -> Self {
        WordDictionary {
            root: Trie::default()
        }
    }

    fn add_word(&mut self, word: String) {
        if word.is_empty() {
            return
        }
        self.root.insert(word.as_str());
    }

    fn search(&self, word: String) -> bool {
        self.root.search(word.as_str())
    }
}

#[test]
fn test() {
    let mut obj = WordDictionary::new();
    obj.add_word("char".to_string());
    assert!(obj.search("char".to_string()));
    assert!(!obj.search("chart".to_string()));
    obj.add_word("chart".to_string());
    assert!(obj.search("chart".to_string()));

    obj.add_word("bad".to_string());
    obj.add_word("dad".to_string());
    obj.add_word("mad".to_string());
    assert!(!obj.search("pad".to_string())); // return False
    assert!(obj.search("bad".to_string())); // return True
    assert!(obj.search(".ad".to_string())); // return True
    assert!(obj.search("b..".to_string())); // return True

}