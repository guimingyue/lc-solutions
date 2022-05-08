use crate::lc::Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut p = 0;
        let mut res = vec![];
        while p < nums.len() {
            if nums[p] as usize != p + 1 {
                let idx = nums[p] as usize - 1;
                if nums[idx] as usize == idx + 1 {
                    res.push(nums[idx]);
                    nums[p] = 0;
                    p += 1;
                } else {
                    if nums[idx] == 0 {
                        nums[idx] = nums[p];
                        nums[p] = 0;
                        p += 1;
                    } else {
                        nums.swap(idx, p);
                    }
                }
            } else {
                p += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![3,15,6,4,8,14,20], Solution::find_duplicates(vec![3,11,8,16,4,15,4,17,14,14,6,6,2,8,3,12,15,20,20,5]));
    assert_eq!(vec![2, 3], Solution::find_duplicates(vec![2,3,2,1,3]));
    assert_eq!(vec![3], Solution::find_duplicates(vec![5,3,2,1,3]));
    assert_eq!(vec![3], Solution::find_duplicates(vec![5,3,3,2,1]));
    assert_eq!(vec![3, 2], Solution::find_duplicates(vec![5,3,3,2,2]));
    assert_eq!(vec![3, 2], Solution::find_duplicates(vec![4,3,3,2,2]));
    assert_eq!(vec![3], Solution::find_duplicates(vec![4,3,3,2,1]));
    assert_eq!(vec![3, 2], Solution::find_duplicates(vec![4,3,2,7,8,2,3,1]));
    assert_eq!(vec![1], Solution::find_duplicates(vec![1,1,2]));
    assert_eq!(vec![1], Solution::find_duplicates(vec![1,1]));
    assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1]));
    assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1, 2, 3]));
}