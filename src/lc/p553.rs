use crate::lc::Solution;

/// #format!
impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() == 1 {
            return format!("{}", nums[0]);
        }
        if nums.len() == 2 {
            return format!("{}/{}", nums[0], nums[1]);
        }
        let mut res = String::new();
        res.push_str(nums[0].to_string().as_str());
        res.push_str("/(");
        for i in 1..nums.len() - 1 {
            res.push_str(nums[i].to_string().as_str());
            res.push_str("/");
        }
        res.push_str(nums[nums.len() - 1].to_string().as_str());
        res.push(')');
        res
    }
}

#[test]
fn test() {
    assert_eq!("1000/(100/10/2)", Solution::optimal_division(vec![1000,100,10,2]));
    assert_eq!("10", Solution::optimal_division(vec![10]));
    assert_eq!("10/5", Solution::optimal_division(vec![10,5]));
}