use crate::lc::Solution;

/// #String
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        fn is_palindrome(s: &str) -> bool {
            let bytes = s.as_bytes();
            let (mut i, mut j) = (0, bytes.len() - 1);
            while i < j {
                if bytes[i] != bytes[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }
        if is_palindrome(s.as_str()) {
            1
        } else {
            2
        }
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::remove_palindrome_sub("ababa".to_string()));
    assert_eq!(2, Solution::remove_palindrome_sub("abb".to_string()));
    assert_eq!(2, Solution::remove_palindrome_sub("baabb".to_string()));
    assert_eq!(2, Solution::remove_palindrome_sub("ababababbababa".to_string()));
    assert_eq!(2, Solution::remove_palindrome_sub("abbab".to_string()));
    assert_eq!(2, Solution::remove_palindrome_sub("abbab".to_string()));
}