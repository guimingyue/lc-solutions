use crate::lc::Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; stations.len() + 1];
        dp[0] = start_fuel;
        for i in 0..stations.len() {
            for j in (0..=i).rev() {
                if dp[j] >= stations[i][0] {
                    dp[j + 1] = dp[j + 1].max(dp[j] + stations[i][1]);
                }
            }
        }

        for i in 0..dp.len() {
            if dp[i] >= target {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::min_refuel_stops(1, 1, vec![]));
    assert_eq!(1, Solution::min_refuel_stops(100, 50, vec![vec![25, 50], vec![50, 25]]));
    assert_eq!(2, Solution::min_refuel_stops(100, 10, vec![vec![10, 60], vec![20, 20], vec![30, 30], vec![60, 40]]));
    assert_eq!(-1, Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]));
    assert_eq!(2, Solution::min_refuel_stops(100, 10, vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]));
}