struct Solution {}

/// #Vec, ^
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0 ;
        for val in nums {
            res = res ^ val;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::single_number(vec![2,2,1]));
    assert_eq!(1, Solution::single_number(vec![-2,-2,1]));
    assert_eq!(4, Solution::single_number(vec![4,1,2,1,2]));
}