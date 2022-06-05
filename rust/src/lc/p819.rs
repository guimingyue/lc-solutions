use crate::lc::Solution;
use std::collections::HashMap;

/// #HashMap
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut map:HashMap<String, i32> = HashMap::new();
        let mut str = String::default();
        let mut max = String::default();
        let mut max_count = 0;
        for (idx, ch) in paragraph.chars().enumerate() {
            if ch.is_ascii_alphabetic() && idx < paragraph.len() - 1 {
                if ch.is_ascii_uppercase() {
                    str.push(ch.to_ascii_lowercase());
                } else {
                    str.push(ch);
                }
            } else {
                if idx == paragraph.len() - 1 && ch.is_ascii_alphabetic() {
                    str.push(ch.to_ascii_lowercase());
                }
                if str.len() == 0 || banned.contains(&str) {
                    str.clear();
                    continue;
                }
                let count = map.entry(str.clone()).or_default();
                *count += 1;
                if *count > max_count {
                    max = str.clone();
                    max_count = *count;
                }
                str.clear();
            }
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!("bob", Solution::most_common_word("was Bob was bob bob".to_string(),
                                                 vec!["hit".to_string(), "ball".to_string()]));
    assert_eq!("bob", Solution::most_common_word("Bob hit a ball, the hit BALL flew far after it was hit. bob.".to_string(),
                                                 vec!["hit".to_string(), "ball".to_string()]));
    assert_eq!("ball", Solution::most_common_word("Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                               vec!["hit".to_string()]));
}