use crate::lc::Solution;

/// #String
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut space = 0;
        let mut res = String::new();
        for ch in s.chars() {
            if ch == ' ' {
                space += 1;
            }
            if space == k {
                break;
            }
            res.push(ch);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("Hello how are you", Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4));
    assert_eq!("What is the solution", Solution::truncate_sentence("What is the solution to this problem".to_string(), 4));
    assert_eq!("chopper is not a tanuki", Solution::truncate_sentence("chopper is not a tanuki".to_string(), 5));
}