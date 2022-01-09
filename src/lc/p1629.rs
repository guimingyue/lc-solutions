use std::cmp::max;
use crate::lc::Solution;

/// #Vec, String
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut res = ' ';
        let mut max_len = 0;
        let mut idx = 0;
        for ch in keys_pressed.chars() {
            if max_len == 0 {
                res = ch;
                max_len = release_times[idx];
            } else {
                let len = release_times[idx] - release_times[idx-1];
                if len > max_len || (len == max_len && ch as u8 > res as u8) {
                    res = ch;
                    max_len = len
                }
            }
            idx += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!('a', Solution::slowest_key(vec![9,17], "ab".to_string()));
    assert_eq!('b', Solution::slowest_key(vec![9,18], "ab".to_string()));
    assert_eq!('c', Solution::slowest_key(vec![9,29,49,50], "cbcd".to_string()));
    assert_eq!('a', Solution::slowest_key(vec![12,23,36,46,62], "spuda".to_string()));
}