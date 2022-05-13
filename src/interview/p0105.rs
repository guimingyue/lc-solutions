use crate::interview::Solution;

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        let (short, long) = if first.len() > second.len() {
            (second.as_bytes(), first.as_bytes())
        } else {
            (first.as_bytes(), second.as_bytes())
        };
        if short.len() + 1 < long.len() {
            return false;
        }
        if short.len() == 0 && long.len() <= 1 {
            return true;
        }
        let (mut i, mut j, mut k) = (0, short.len() - 1, long.len() - 1);
        while i <= j && short[i] == long[i]{
            i += 1;
        }
        if i > j {
            return true;
        }
        while j > i && short[j] == long[k] {
            j -= 1;
            k -= 1;
        }
        if i == k {
            true
        } else {
            short[i] == long[i] || short[j] == long[k]
        }
    }
}

#[test]
fn test() {
    assert!(!Solution::one_edit_away("ab".to_string(), "bc".to_string()));
    assert!(Solution::one_edit_away("".to_string(), "".to_string()));
    assert!(Solution::one_edit_away("a".to_string(), "".to_string()));
    assert!(Solution::one_edit_away("abc".to_string(), "adc".to_string()));
    assert!(Solution::one_edit_away("abc".to_string(), "ac".to_string()));
    assert!(Solution::one_edit_away("a".to_string(), "ac".to_string()));
    assert!(Solution::one_edit_away("ac".to_string(), "ac".to_string()));
    assert!(Solution::one_edit_away("ac".to_string(), "a".to_string()));
    assert!(Solution::one_edit_away("a".to_string(), "b".to_string()));
    assert!(Solution::one_edit_away("pale".to_string(), "ple".to_string()));
    assert!(!Solution::one_edit_away("pales".to_string(), "pal".to_string()));
    assert!(Solution::one_edit_away("pal".to_string(), "pale".to_string()));
}