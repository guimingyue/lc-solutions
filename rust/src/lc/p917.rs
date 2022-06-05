use crate::lc::Solution;

/// #Vec.swap
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut bytes = s.into_bytes();
        let (mut i, mut j) = (0, bytes.len() - 1);
        while i < j {
            while i < j && !(bytes[i] as char).is_ascii_alphabetic() {
                i += 1;
            }
            while i < j && !(bytes[j] as char).is_ascii_alphabetic() {
                j -= 1;
            }
            bytes.swap(i, j);
            i += 1;
            j -= 1;
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!("7_28]", Solution::reverse_only_letters("7_28]".to_string()));
    assert_eq!("dc-ba", Solution::reverse_only_letters("ab-cd".to_string()));
    assert_eq!("j-Ih-gfE-dCba", Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()));
    assert_eq!("Qedo1ct-eeLg=ntse-T!", Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()));
}