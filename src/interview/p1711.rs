use std::collections::HashMap;
use crate::lc::Solution;

impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut map:HashMap<&str, Vec<usize>> = HashMap::new();
        for (idx, w) in words.iter().enumerate() {
            let list = map.entry(w).or_default();
            list.push(idx);
        }
        let l1 = map.get(word1.as_str());
        let l2 = map.get(word2.as_str());
        if l1.is_none() || l2.is_none() {
            return i32::MAX;
        }
        let v1 = l1.unwrap();
        let v2 = l2.unwrap();
        let mut res = i32::MAX;
        let (mut i, mut j) = (0, 0);
        while i < v1.len() && j < v2.len() {
            let dis = (v1[i] as i32 - v2[j] as i32).abs();
            if dis < res {
                res = dis;
            }
            if v1[i] < v2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        while i < v1.len() {
            let dis = (v1[i] as i32 - v2[v2.len() - 1] as i32).abs();
            if dis < res {
                res = dis;
            }
            i += 1;
        }
        while j < v2.len() {
            let dis = (v1[v1.len() - 1] as i32 - v2[j] as i32).abs();
            if dis < res {
                res = dis;
            }
            j += 1;
        }
        res
    }
}

#[test]
fn test() {
    fn test(vec: Vec<&str>, word1: &str, word2: &str) -> i32 {
        let words = vec.iter().map(|str|str.to_string()).collect();
        Solution::find_closest(words, word1.to_string(), word2.to_string())
    }
    assert_eq!(1, test(vec!["I","am","a","student","from","a","university","in","a","city"], "a", "student"));
    assert_eq!(1, test(vec!["a","student","a","student","from","a","university","in","a","city"], "a", "student"));
    assert_eq!(1, test(vec!["a","a","a","student"], "a", "student"));
}