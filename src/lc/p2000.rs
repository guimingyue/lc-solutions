use crate::lc::Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let bytes = word.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] == ch as u8 {
                let mut res = String::new();
                for j in (0..=i).rev() {
                    res.push(bytes[j] as char);
                }
                for j in i+1..bytes.len() {
                    res.push(bytes[j] as char);
                }
                return res;
            }
        }
        word
    }
}

#[test]
fn test() {
    assert_eq!("dcbaefd", Solution::reverse_prefix("abcdefd".to_string(), 'd'));
    assert_eq!("zxyxxe", Solution::reverse_prefix("xyxzxe".to_string(), 'z'));
    assert_eq!("abcd", Solution::reverse_prefix("abcd".to_string(), 'z'));
    assert_eq!("dcba", Solution::reverse_prefix("abcd".to_string(), 'd'));
    assert_eq!("cbaba", Solution::reverse_prefix("abcba".to_string(), 'c'));
    assert_eq!("a", Solution::reverse_prefix("a".to_string(), 'a'));
}