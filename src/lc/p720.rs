use std::collections::HashSet;
use crate::lc::Solution;

/// #HashSet, String.gt
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut set = HashSet::new();
        for word in &words {
            set.insert(word.as_str());
        }

        let mut res = String::new();
        for word in &words {
            let mut exist = true;
            for i in (0..word.len() - 1).rev() {
                if !set.contains(&word[0..=i]) {
                    exist = false;
                    break;
                }
            }
            if !exist {
                continue;
            }
            if res.is_empty() || res.len() < word.len() || (res.len() == word.len() && res.gt(word)) {
                res.clear();
                res.push_str(word.as_str());
            }

        }
        res
    }
}

#[test]
fn test() {
    fn execute(vec: Vec<&str>) -> String {
        let new_vec = vec.iter().map(|x|x.to_string()).collect();
        Solution::longest_word(new_vec)
    }
    assert_eq!("world", execute(vec!["w","wo","wor","worl", "world"]));
    assert_eq!("apple", execute(vec!["a", "banana", "app", "appl", "ap", "apply", "apple"]));
}