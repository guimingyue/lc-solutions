use crate::lc::Solution;
use std::collections::HashMap;

/// #HashMap
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut map:HashMap<char, i32> = HashMap::new();
        for ch in  "balloon".chars() {
            map.entry(ch).or_default();
        }
        for ch in text.chars() {
            if map.contains_key(&ch) {
                let num = map.entry(ch).or_default();
                *num += 1;
            }
        }
        let mut res = i32::MAX;
        for (ch, num) in map {
            if ch == 'l' || ch == 'o' {
                res = res.min(num / 2);
            } else {
                res = res.min(num);
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::max_number_of_balloons("nlaebolko".to_string()));
    assert_eq!(2, Solution::max_number_of_balloons("loonbalxballpoon".to_string()));
    assert_eq!(0, Solution::max_number_of_balloons("leetcode".to_string()));
}