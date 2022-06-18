use std::collections::HashMap;
use crate::lc::Solution;

/// #HashMap
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for v in nums {
            let e = map.entry(v).or_default();
            *e += 1;
        }
        let mut res = 0;

        for v in map.keys() {
            if k == 0 {
                if map[v] > 1 {
                    res += 1;
                }
            } else {
                let next = *v + k;
                if !map.contains_key(&next) {
                    continue;
                }
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::find_pairs(vec![3, 1, 4, 1, 5], 2));
    assert_eq!(4, Solution::find_pairs(vec![1, 2, 3, 4, 5], 1));
    assert_eq!(1, Solution::find_pairs(vec![1, 3, 1, 5, 4], 0));
}