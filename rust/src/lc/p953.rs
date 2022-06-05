use std::collections::HashMap;
use crate::lc::Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = HashMap::new();
        for (idx, ch) in order.chars().enumerate() {
            map.insert(ch as u8, idx);
        }

        for i in 1..words.len() {
            let pre = words[i-1].as_bytes();
            let cur = words[i].as_bytes();
            let mut i = 0;
            while i < pre.len() && i < cur.len() {
                if map[&pre[i]] != map[&cur[i]] {
                    break;
                }
                i += 1;
            }
            if i >= cur.len() && i < pre.len() {
                return false;
            }
            if i < pre.len() && i < cur.len() && map[&pre[i]] > map[&cur[i]]{
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::is_alien_sorted(vec!["hello".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()));
    assert!(Solution::is_alien_sorted(vec!["hello".to_string(),"leetcode".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()));
    assert!(!Solution::is_alien_sorted(vec!["word".to_string(),"world".to_string(),"row".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()));
    assert!(!Solution::is_alien_sorted(vec!["apple".to_string(),"app".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()));
}