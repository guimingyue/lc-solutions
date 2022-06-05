use crate::lc::Solution;

/// #log2, ceil
impl Solution {

    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let states = minutes_to_test / minutes_to_die + 1;
        ((buckets as f64).log2() / (states as f64).log2()).ceil() as i32
    }
}

#[test]
fn test() {
    assert_eq!(5, Solution::poor_pigs(1000, 15, 60));
    assert_eq!(2, Solution::poor_pigs(4, 15, 30));
    assert_eq!(2, Solution::poor_pigs(4, 15, 15));
}