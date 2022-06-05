use std::collections::{HashMap, HashSet};
use crate::lc::Solution;

/// #HashMap, HashSet
impl Solution {

    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn dfs(nums: &Vec<i32>, idx: usize, xor: i32, max_or: &mut i32, res: &mut i32) {
            if idx == nums.len() {
                if xor > *max_or {
                    *max_or = xor;
                    *res = 1;
                } else if xor == *max_or {
                    *res += 1;
                }
                return;
            }
            dfs(nums, idx + 1, xor, max_or, res);
            dfs(nums, idx + 1, xor | nums[idx], max_or, res);
        }
        let mut res = 0;
        let mut max_or = 0;
        dfs(&nums, 0, 0, &mut max_or, &mut res);
        res
    }

    pub fn count_max_or_subsets_(nums: Vec<i32>) -> i32 {
        const a:[char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        fn dfs(nums: &Vec<i32>, idx: usize, xor: i32, idx_str: &mut String, expect: i32, set: &mut HashSet<String>) {
            if xor == expect {
                let str = idx_str.clone();
                if !set.contains(idx_str) {
                    set.insert(str);
                }

            }
            if idx == nums.len() {
                return;
            }
            dfs(nums, idx + 1, xor, idx_str, expect, set);
            idx_str.push(a[idx]);
            dfs(nums, idx + 1, xor | nums[idx], idx_str, expect, set);
            idx_str.remove(idx_str.len() - 1);
        }

        let mut expect = 0;
        for v in &nums {
            expect |= *v;
        }
        let mut idx_str = String::new();
        let mut set = HashSet::new();
        dfs(&nums, 0, 0, &mut idx_str, expect, &mut set);
        set.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::count_max_or_subsets(vec![3, 1]));
    assert_eq!(7, Solution::count_max_or_subsets(vec![2,2,2]));
    assert_eq!(6, Solution::count_max_or_subsets(vec![3,2,1,5]));
}