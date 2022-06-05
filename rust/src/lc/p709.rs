use crate::lc::Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        const base:u8 = 'a' as u8 - 'A' as u8;
        let mut res = String::new();
        for ch in s.chars() {
            let c = match ch {
                'A'..='Z' =>
                    (ch as u8 + base) as char,
                _ =>
                    ch
            };
            res.push(c);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("hello", Solution::to_lower_case("Hello".to_string()));
    assert_eq!("here", Solution::to_lower_case("here".to_string()));
    assert_eq!("lovely", Solution::to_lower_case("LOVELY".to_string()));
}