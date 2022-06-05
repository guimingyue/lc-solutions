use crate::lc::Solution;

use std::collections::HashMap;

/// #HashMap, String, split
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let s1_split:Vec<_> = s1.split(" ").map(|s|s.trim()).filter(|s|s.len() > 0).collect();
        let mut map:HashMap<&str, i32> = HashMap::new();
        for i in 0..s1_split.len() {
            let count = map.entry(s1_split[i]).or_default();
            *count += 1;
        }

        let s2_split:Vec<_> = s2.split(" ").map(|s|s.trim()).filter(|s|s.len() > 0).collect();
        for i in 0..s2_split.len() {
            let count = map.entry(s2_split[i]).or_default();
            *count += 1;
        }

        let mut res = vec![];
        for (k, v) in map {
            if v == 1 {
                res.push(k.to_string());
            }
        }
        res
    }
}

#[test]
fn test() {
    fn assert(expect: &mut Vec<String>, actual: &mut Vec<String>) {
        expect.sort();
        actual.sort();
        assert_eq!(expect, actual);
    }
    assert(&mut vec!["sweet".to_string(), "sour".to_string()],
               &mut Solution::uncommon_from_sentences("this apple is  sweet".to_string(), "this apple is sour".to_string()));
    assert(&mut vec!["banana".to_string()],
               &mut Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()));
}