use crate::lc::Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        while num >= 10 {
            let mut temp = 0;
            while num > 0 {
                temp += num % 10;
                num /= 10;
            }
            num = temp;
        }
        num
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::add_digits(38));
    assert_eq!(1, Solution::add_digits(i32::MAX));
}