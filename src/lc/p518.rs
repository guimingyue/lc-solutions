use crate::lc::Solution;

/// #Vec
impl Solution {

    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        for v in coins {
            for num in v..=amount {
                dp[num as usize] += dp[(num - v) as usize];
            }
        }
        dp[amount as usize]
    }

    pub fn change_tle(amount: i32, coins: Vec<i32>) -> i32 {
        fn dfs(total: i32, coins: &Vec<i32>, idx: i32, res: &mut i32) {
            if total == 0 {
                *res += 1;
                return;
            }
            if idx < 0 {
                return;
            }
            for i in (0..=idx).rev() {
                let coin = coins[i as usize];
                if total < coin {
                    continue;
                }
                dfs(total - coin, coins, i, res);
            }
        }
        let mut res = 0;
        dfs(amount, &coins, (coins.len() - 1) as i32, &mut res);
        res
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
    assert_eq!(0, Solution::change(3, vec![2]));
    assert_eq!(1, Solution::change(10, vec![10]));
    assert_eq!(35502874, Solution::change(500, vec![3,5,7,8,9,10,11]));
}