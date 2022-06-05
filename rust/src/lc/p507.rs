use crate::lc::Solution;

/// #sqrt
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let sqrt = (num as f64).sqrt() as i32;
        let mut i = 2;
        let mut sum = 1;
        while i <= sqrt {
            if num % i == 0 {
                sum += i + num / i;
            }
            i += 1;
        }
        num > 1 && sum == num
    }
}

#[test]
fn test() {
    assert!(!Solution::check_perfect_number(1));
    assert!(Solution::check_perfect_number(28));
    assert!(Solution::check_perfect_number(6));
    assert!(Solution::check_perfect_number(496));
    assert!(Solution::check_perfect_number(8128));
    assert!(!Solution::check_perfect_number(2));
}