use crate::lc::Solution;

/// #&
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        (n > 0) && (n & (n-1) == 0)
    }
}

#[test]
fn test() {
    assert!(Solution::is_power_of_two(1));
    assert!(Solution::is_power_of_two(2));
    assert!(!Solution::is_power_of_two(3));
    assert!(Solution::is_power_of_two(4));
    assert!(!Solution::is_power_of_two(1023));
    assert!(Solution::is_power_of_two(1024));
    assert!(!Solution::is_power_of_two(-1));
    assert!(!Solution::is_power_of_two(-2));
}