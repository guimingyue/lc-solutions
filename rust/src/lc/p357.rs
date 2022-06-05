use crate::lc::Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut res = 1;
        let mut k = 1;
        let mut cur = 1;
        let mut choice = 9;
        while k <= n {
            cur *= choice;
            if k != 1 {
                choice -= 1;
            }
            k += 1;
            res += cur;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::count_numbers_with_unique_digits(0));
    assert_eq!(10, Solution::count_numbers_with_unique_digits(1));
    assert_eq!(91, Solution::count_numbers_with_unique_digits(2));
}