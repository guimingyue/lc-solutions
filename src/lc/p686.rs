use crate::lc::Solution;

/// #String, contains
impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut res = 0;
        let mut str = String::new();
        while str.len() < b.len() {
            str.push_str(a.as_str());
            res += 1;
        }
        if str.contains(b.as_str()) {
            return res;
        }
        str.push_str(a.as_str());
        if str.contains(b.as_str()) {
            return res + 1;
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()));
    assert_eq!(2, Solution::repeated_string_match("a".to_string(), "aa".to_string()));
    assert_eq!(1, Solution::repeated_string_match("a".to_string(), "a".to_string()));
    assert_eq!(-1, Solution::repeated_string_match("abc".to_string(), "wxyz".to_string()));
}