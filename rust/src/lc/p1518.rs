use crate::lc::Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num = num_bottles;
        let mut res = 0;
        let mut num_empty_bottle = 0;
        while num  > 0 {
            res += num;
            num_empty_bottle += num;
            num = num_empty_bottle / num_exchange;
            num_empty_bottle -= num * num_exchange;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(13, Solution::num_water_bottles(9, 3));
    assert_eq!(19, Solution::num_water_bottles(15, 4));
    assert_eq!(6, Solution::num_water_bottles(5, 5));
    assert_eq!(2, Solution::num_water_bottles(2, 3));
}