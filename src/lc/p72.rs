use crate::lc::Solution;

/// #Vec, String, min
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len()+1]; word1.len()+1];
        for i in 0..=word1.len() {
            dp[i][word2.len()] = word1.len() - i;
        }
        for i in 0..=word2.len() {
            dp[word1.len()][i] = word2.len() - i;
        }
        for (i, v1) in word1.char_indices().rev() {
            for (j, v2) in word2.char_indices().rev() {
                if v1 == v2 {
                    dp[i][j] = dp[i+1][j+1];
                } else {
                    dp[i][j] = std::cmp::min(std::cmp::min(dp[i+1][j], dp[i][j+1]), dp[i+1][j+1]) + 1;
                }
            }
        }
        dp[0][0] as i32
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::min_distance("horse".to_string(), "ros".to_string()));
    assert_eq!(5, Solution::min_distance("intention".to_string(), "execution".to_string()));
    assert_eq!(9, Solution::min_distance("".to_string(), "execution".to_string()));
    assert_eq!(1, Solution::min_distance("".to_string(), "e".to_string()));
}