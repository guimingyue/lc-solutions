struct Solution {}

/// #Vec, HashMap
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut map = std::collections::HashMap::new();
        for val in nums2.iter().rev() {
            while !stack.is_empty() && *stack.last().unwrap() < *val {
                stack.pop();
            }
            if stack.is_empty() {
                map.insert(*val, -1);
            } else {
                map.insert(*val, *stack.last().unwrap());
            }
            stack.push(*val);
        }

        let mut res = vec![];
        for val in nums1.iter() {
            res.push(map[val]);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![-1, 3, -1], Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]));
    assert_eq!(vec![-1, 4, 3], Solution::next_greater_element(vec![2, 3, 1], vec![1, 3, 4, 2]));
    assert_eq!(vec![3, -1], Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]));
    assert_eq!(vec![2, 3, 4], Solution::next_greater_element(vec![1, 2, 3], vec![1, 2, 3, 4]));
}