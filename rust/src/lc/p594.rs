use crate::lc::Solution;

/// #BTreeMap, max
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::BTreeMap::<i32,i32>::new();
        nums.iter().for_each(|x| {
           let num = map.entry(*x).or_default();
            *num = *num + 1;
        });
        let mut res = 0;
        let keys: Vec<_> = map.keys().clone().collect();
        for i in 0..keys.len()-1 {
            if *keys[i+1] - *keys[i] == 1 {
                let sum = map.get(keys[i]).unwrap() + map.get(keys[i+1]).unwrap();
                res = std::cmp::max(res, sum);
            }
        }
        res

    }
}

#[test]
fn test() {
    assert_eq!(5, Solution::find_lhs(vec![1,3,2,2,5,2,3,7]));
    assert_eq!(2, Solution::find_lhs(vec![1,2,3,4]));
    assert_eq!(0, Solution::find_lhs(vec![1,1,1,1]));
    assert_eq!(0, Solution::find_lhs(vec![3]));
    assert_eq!(2, Solution::find_lhs(vec![0,-1,1]));
}