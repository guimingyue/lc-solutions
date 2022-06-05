use crate::lc::Solution;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        fn is_consecutive(k: i32, n: i32) -> bool {
            if k % 2 == 1 {
                n % k == 0
            } else {
                n % k != 0 && (2 * n) % k == 0
            }
        }
        let mut k = 1;
        let mut res = 0;
        while k * (k + 1) <= 2 * n {
            if is_consecutive(k, n) {
                res += 1;
            }
            k += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::consecutive_numbers_sum(5));
    assert_eq!(3, Solution::consecutive_numbers_sum(9));
    assert_eq!(4, Solution::consecutive_numbers_sum(15));
    assert_eq!(2, Solution::consecutive_numbers_sum(6));
}