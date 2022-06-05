use crate::lc::Solution;

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let bytes = p.as_bytes();
        let mut dp = [0; 26];
        let mut k = 1;
        for i in 0..bytes.len() {
            if i > 0 && (bytes[i] as i32 - bytes[i-1] as i32 + 26) % 26 == 1 {
                k += 1;
            } else {
                k = 1;
            }
            let idx = (bytes[i] - 'a' as u8) as usize;
            dp[idx] = k.max(dp[idx]);
        }
        dp.iter().sum()
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::find_substring_in_wrapround_string("cac".to_string()));
    assert_eq!(1, Solution::find_substring_in_wrapround_string("a".to_string()));
    assert_eq!(3, Solution::find_substring_in_wrapround_string("ab".to_string()));
    assert_eq!(6, Solution::find_substring_in_wrapround_string("zab".to_string()));
}