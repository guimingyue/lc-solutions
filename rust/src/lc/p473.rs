use crate::lc::Solution;

/// #<<, !
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let avg = sum / 4;
        let n = matchsticks.len();
        let mut dp = vec![-1; 1 << n];
        dp[0] = 0;
        for s in 1..1 << n {
            for k in 0..n {
                if s & (1 << k) == 0 {
                    continue;
                }
                let s1 = s & !(1 << k);
                if dp[s1] >= 0 && dp[s1] + matchsticks[k] <= avg {
                    dp[s] = (dp[s1] + matchsticks[k]) % avg;
                    break;
                }
            }
        }
        dp[(1 << n) - 1] == 0
    }
}

#[test]
fn test() {
    assert!(Solution::makesquare(vec![1,1,2,2,2]));
    assert!(Solution::makesquare(vec![1,2,1,2,3,3,4]));
    assert!(!Solution::makesquare(vec![1,2,3,3,3,4]));
    assert!(!Solution::makesquare(vec![3,3,3,3,4]));
}