use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut vec = vec![0;26];
        let base = 'a' as u8;
        for c in magazine.into_bytes() {
            vec[(c - base) as usize] += 1;
        }
        for c in ransom_note.into_bytes() {
            vec[(c - base) as usize] -= 1;
            if vec[(c - base) as usize] < 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
}