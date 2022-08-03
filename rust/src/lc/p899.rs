use std::cmp::Ordering;
use crate::lc::Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s;
        let mut bytes = s.into_bytes();
        let len = bytes.len();
        if k == 1 {
            let mut smallest = bytes.clone();
            for i in 0..len {
                let v = bytes.remove(0);
                bytes.push(v);
                if smallest.cmp(&bytes) == Ordering::Greater {
                    smallest = bytes.clone();
                }
            }
            bytes = smallest;
        } else {
            bytes.sort();
        }
        unsafe {String::from_utf8_unchecked(bytes)}
    }
}

#[test]
fn test() {
    assert_eq!("hku", Solution::orderly_queue("kuh".to_string(), 1));
    assert_eq!("acb", Solution::orderly_queue("cba".to_string(), 1));
    assert_eq!("aaabc", Solution::orderly_queue("baaca".to_string(), 3));
}