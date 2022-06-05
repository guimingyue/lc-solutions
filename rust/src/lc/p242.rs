use crate::lc::Solution;

/// #String
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s_bytes = s.into_bytes();
        let t_bytes = t.into_bytes();
        let base = 'a' as u8;
        let mut arr = [0;26];
        for i in 0..s_bytes.len() {
            arr[(s_bytes[i] - base) as usize] += 1;
            arr[(t_bytes[i] - base) as usize] -= 1;
        }
        for n in arr {
            if n != 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
}