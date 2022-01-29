use crate::lc::Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut res = 0;
        let mut num = n;
        while num > 1 {
            let t = num / 2;
            res += t;
            num = t + num % 2;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(6, Solution::number_of_matches(7));
    assert_eq!(13, Solution::number_of_matches(14));
}