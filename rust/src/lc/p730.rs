use crate::lc::Solution;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let mut dp = vec![vec![vec![0; s.len()]; s.len()]; 4];
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            dp[(bytes[i] - 'a' as u8) as usize][i][i] = 1;
        }

        let (ch_start, ch_end) = ('a' as u8, 'd' as u8);
        for len in 2..=bytes.len() {
            for i in 0..=bytes.len() - len {
                let j = i + len - 1;
                for ch in ch_start..=ch_end {
                    let k = (ch - 'a' as u8) as usize;
                    if ch == bytes[i] && bytes[i] == bytes[j] {
                        dp[k][i][j] = (2 + (dp[0][i+1][j-1] + dp[1][i+1][j-1]) % MOD + (dp[2][i+1][j-1] + dp[3][i+1][j-1]) % MOD) % MOD;
                    } else if ch == bytes[i] {
                        dp[k][i][j] = dp[k][i][j-1];
                    } else if ch == bytes[j] {
                        dp[k][i][j] = dp[k][i+1][j];
                    } else {
                        dp[k][i][j] = dp[k][i+1][j-1];
                    }
                }
            }
        }
        let mut res = 0;
        for i in 0..dp.len() {
            res = (res + dp[i][0][bytes.len()-1]) % MOD;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(6, Solution::count_palindromic_subsequences("bccb".to_string()));
    assert_eq!(104860361, Solution::count_palindromic_subsequences("abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string()));
}