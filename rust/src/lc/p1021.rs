use crate::lc::Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut len = -1;
        let mut res = String::new();
        for ch in s.chars() {
            if ch == '(' {
                if len >= 0 {
                    res.push(ch);
                }
                len += 1;
            } else {
                len -= 1;
                if len >= 0 {
                    res.push(ch);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("()()()", Solution::remove_outer_parentheses("(()())(())".to_string()));
    assert_eq!("", Solution::remove_outer_parentheses("()()".to_string()));
    assert_eq!("", Solution::remove_outer_parentheses("()()()".to_string()));
    assert_eq!("()()()", Solution::remove_outer_parentheses("(()()())".to_string()));
    assert_eq!("()()()()(())", Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()));
}