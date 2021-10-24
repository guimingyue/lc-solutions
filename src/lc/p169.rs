struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut elem = 0;
        let mut vote = 0;
        for val in nums.iter() {
            if vote > 0 && *val == elem {
                vote += 1;
            } else if vote == 0 {
                elem = *val;
                vote += 1;
            } else {
                vote -= 1;
            }
        }
        elem
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::majority_element(vec![3,2,3]));
    assert_eq!(2, Solution::majority_element(vec![2]));
    assert_eq!(2, Solution::majority_element(vec![2,2,1,1,1,2,2]));
}