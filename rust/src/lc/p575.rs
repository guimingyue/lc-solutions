use crate::lc::Solution;

/// #HashMap, std::cmp::min
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut total = candy_type.len();
        for v in candy_type {
            if let Some(&cur) = map.get(&v) {
                map.insert(v, cur+1);
            } else {
                map.insert(v, 1);
            }
        }
        std::cmp::min(total / 2, map.len()) as i32
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::distribute_candies(vec![1,1,2,2,3,3]));
    assert_eq!(2, Solution::distribute_candies(vec![1,1,2,3]));
    assert_eq!(3, Solution::distribute_candies(vec![1,1,2,2,2,3]));
    assert_eq!(2, Solution::distribute_candies(vec![1,2,3,4]));
    assert_eq!(1, Solution::distribute_candies(vec![1,2]));
    assert_eq!(1, Solution::distribute_candies(vec![-1,-2]));
    assert_eq!(3, Solution::distribute_candies(vec![-1,1,2,2,3,3]));
    assert_eq!(3, Solution::distribute_candies(vec![-1,-2,2,2,3,3]));
}