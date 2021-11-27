use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn product_except_self_mon(nums: Vec<i32>) -> Vec<i32> {
        let mut l = vec![1; nums.len()];
        let mut r = vec![1; nums.len()];
        for i in 1..nums.len() {
            l[i] = nums[i-1] * l[i-1];
        }
        for i in (0..nums.len()-1).rev() {
            r[i] = nums[i+1] * r[i+1];
        }
        let mut res = vec![];
        for i in 0..nums.len() {
            res.push(l[i] * r[i]);
        }
        res
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        for i in 1..nums.len() {
            res[i] = nums[i-1] * res[i-1];
        }

        let mut r = 1;

        for i in (0..nums.len()).rev() {
            res[i] = res[i] * r;
            r *= nums[i];
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![24,12,8,6], Solution::product_except_self(vec![1,2,3,4]));
    assert_eq!(vec![6,8,12,24], Solution::product_except_self(vec![4,3,2,1]));
    assert_eq!(vec![2,1], Solution::product_except_self(vec![1,2]));
}