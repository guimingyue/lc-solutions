use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let (mut max_idx, mut max, mut mid) = (-1, 0, 0);
        for (i, v) in nums.iter().enumerate() {
            if *v > max {
                let temp = max;
                max = *v;
                max_idx = i as i32;
                if temp > mid {
                    mid = temp;
                }
            } else if *v > mid {
                mid = *v;
            }
        }
        if max < mid + mid {
            max_idx = -1;
        }
        max_idx
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::dominant_index(vec![0,0,0,1]));
    assert_eq!(1, Solution::dominant_index(vec![3,6,1,0]));
    assert_eq!(-1, Solution::dominant_index(vec![1,2,3,4]));
    assert_eq!(0, Solution::dominant_index(vec![1]));
    assert_eq!(1, Solution::dominant_index(vec![1,2]));
    assert_eq!(-1, Solution::dominant_index(vec![2,3]));
}