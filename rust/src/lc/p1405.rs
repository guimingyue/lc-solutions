use crate::lc::Solution;
use std::collections::BinaryHeap;

/// #BinaryHeap, Vec
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut priority = BinaryHeap::new();
        priority.push((a, 'a'));
        priority.push((b, 'b'));
        priority.push((c, 'c'));
        let mut vec = Vec::new();
        let mut bak = None;
        while let Some((num, ch)) = priority.pop() {
            if num <= 0 {
                continue;
            }
            if vec.len() >= 2 && vec[vec.len()-1] == ch as u8 && vec[vec.len() - 2] == ch as u8 {
                bak = Some((num, ch));
            } else {
                vec.push(ch as u8);
                if num > 1 {
                    priority.push((num - 1, ch));
                }
                if let Some((n, v)) = bak {
                    priority.push((n, v));
                    bak = None;
                }
            }
        }
        String::from_utf8(vec).unwrap_or(String::new())
    }
}

#[test]
fn test() {
    assert_eq!("bbabcba", Solution::longest_diverse_string(2, 4, 1));
    assert_eq!("ccbccacc", Solution::longest_diverse_string(1, 1, 7));
    assert_eq!("bacba", Solution::longest_diverse_string(2, 2, 1));
    assert_eq!("aabaa", Solution::longest_diverse_string(7, 1, 0));
}