use crate::lc::Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let seven_num = n / 7;
        let seven_mod = n % 7;
        28 * seven_num + 7 * seven_num * (seven_num - 1) / 2 + (seven_num + 1 + seven_num + seven_mod) * seven_mod / 2
    }
}

#[test]
fn test() {
    assert_eq!(10, Solution::total_money(4));
    assert_eq!(37, Solution::total_money(10));
    assert_eq!(96, Solution::total_money(20));
}