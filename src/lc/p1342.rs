use crate::lc::Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num;
        let mut step = 0;
        while n > 0 {
            if n & 1 == 1 {
                n = n - 1
            } else {
                n = n >> 1;
            }
            step += 1;
        }
        step
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::number_of_steps(5));
    assert_eq!(1, Solution::number_of_steps(1));
    assert_eq!(2, Solution::number_of_steps(2));
    assert_eq!(3, Solution::number_of_steps(3));
    assert_eq!(3, Solution::number_of_steps(4));
    assert_eq!(4, Solution::number_of_steps(6));
    assert_eq!(5, Solution::number_of_steps(7));
    assert_eq!(4, Solution::number_of_steps(8));
    assert_eq!(5, Solution::number_of_steps(9));
    assert_eq!(6, Solution::number_of_steps(14));
    assert_eq!(12, Solution::number_of_steps(123));
}