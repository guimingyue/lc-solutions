use crate::lc::Solution;

/// #String
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut first_upper = false;
        let mut contain_lower = false;
        let mut not_first_upper = false;
        for (idx, ch) in word.char_indices() {
            if idx == 0 {
                first_upper = ch.is_uppercase();
                contain_lower = ch.is_lowercase();
                continue;
            }
            if ch.is_lowercase() {
                contain_lower = true;
            }
            if ch.is_uppercase() {
                not_first_upper = true;
            }
            if first_upper && contain_lower && ch.is_uppercase() {
                return false;
            }
            if !first_upper && ch.is_uppercase() {
                return false;
            }
            if first_upper && contain_lower && not_first_upper {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::detect_capital_use("USA".to_string()));
    assert!(!Solution::detect_capital_use("USa".to_string()));
    assert!(!Solution::detect_capital_use("FlaG".to_string()));
    assert!(!Solution::detect_capital_use("FLag".to_string()));
    assert!(Solution::detect_capital_use("leetcode".to_string()));
    assert!(Solution::detect_capital_use("Leetcode".to_string()));
    assert!(!Solution::detect_capital_use("LeetCode".to_string()));
    assert!(Solution::detect_capital_use("Google".to_string()));
    assert!(Solution::detect_capital_use("google".to_string()));
    assert!(Solution::detect_capital_use("A".to_string()));
    assert!(Solution::detect_capital_use("a".to_string()));
    assert!(Solution::detect_capital_use("Ab".to_string()));
    assert!(Solution::detect_capital_use("ab".to_string()));
    assert!(!Solution::detect_capital_use("aB".to_string()));
}