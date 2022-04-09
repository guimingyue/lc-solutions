use crate::lc::Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut double = String::from(s.as_str());
        double.push_str(s.as_str());
        double.contains(goal.as_str())
    }
}

#[test]
fn test() {
    assert!(Solution::rotate_string("abcde".to_string(), "cdeab".to_string()));
    assert!(!Solution::rotate_string("abcde".to_string(), "abced".to_string()));
}