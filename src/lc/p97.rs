use crate::lc::Solution;

/// #Vec, String
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                if i > 0 {
                    dp[i][j] = dp[i][j] || (dp[i-1][j] && s1.as_bytes()[i-1] == s3.as_bytes()[i+j-1])
                }
                if j > 0 {
                    dp[i][j] = dp[i][j] || (dp[i][j-1] && s2.as_bytes()[j-1] == s3.as_bytes()[i+j-1])
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
}

#[test]
fn test() {
    assert!(Solution::is_interleave("".to_string(), "".to_string(), "".to_string()));
    assert!(!Solution::is_interleave("".to_string(), "".to_string(), "a".to_string()));
    assert!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()));
    assert!(!Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()));
}