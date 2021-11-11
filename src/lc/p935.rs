use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let mut dp = vec![vec![1;10], vec![0;10]];
        let moves = vec![vec![4, 6], vec![6, 8], vec![7, 9], vec![4, 8], vec![0, 3, 9], vec![], vec![0, 1, 7], vec![2, 6], vec![1, 3], vec![2, 4]];
        for i in 1..n {
            dp[(i % 2) as usize].fill(0);
            for j in 0..10 {
                for m in moves[j].iter() {
                    let next = *m;
                    dp[(i % 2) as usize][next] += dp[((i - 1) % 2) as usize][j];
                    dp[(i % 2) as usize][next] %= MOD;
                }
            }
        }
        let mut res = 0;
        for x in dp[((n-1) % 2) as usize].iter() {
            res += *x;
            res %= MOD;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(10, Solution::knight_dialer(1));
    assert_eq!(20, Solution::knight_dialer(2));
    assert_eq!(46, Solution::knight_dialer(3));
    assert_eq!(104, Solution::knight_dialer(4));
    assert_eq!(136006598, Solution::knight_dialer(3131));
}