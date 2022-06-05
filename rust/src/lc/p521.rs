use crate::lc::Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a.eq(&b) {
            -1
        } else {
            a.len().max(b.len()) as i32
        }
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::find_lu_slength("aba".to_string(), "cbc".to_string()));
    assert_eq!(3, Solution::find_lu_slength("aba".to_string(), "cdc".to_string()));
    assert_eq!(3, Solution::find_lu_slength("aba".to_string(), "aca".to_string()));
    assert_eq!(3, Solution::find_lu_slength("aba".to_string(), "abc".to_string()));
    assert_eq!(3, Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()));
    assert_eq!(3, Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()));
    assert_eq!(-1, Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()));
}