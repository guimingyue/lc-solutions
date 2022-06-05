use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut total = 0;
        for i in 0..(time_series.len() - 1) {
            if time_series[i] + duration < time_series[i+1] {
                total += duration;
            } else {
                total += time_series[i+1] - time_series[i];
            }
        }
        total + duration
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::find_poisoned_duration(vec![1, 4], 2));
    assert_eq!(3, Solution::find_poisoned_duration(vec![1, 2], 2));
    assert_eq!(4, Solution::find_poisoned_duration(vec![1, 3], 2));
    assert_eq!(2, Solution::find_poisoned_duration(vec![1, 1], 2));
}