use crate::lc::Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::bulb_switch(1));
    assert_eq!(1, Solution::bulb_switch(2));
    assert_eq!(1, Solution::bulb_switch(3));
    assert_eq!(2, Solution::bulb_switch(4));
    assert_eq!(3, Solution::bulb_switch(9));
}