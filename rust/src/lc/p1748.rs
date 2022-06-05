use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut vec = vec![0;101];
        let mut res = 0;
        for num in nums {
            if vec[num as usize] == 0 {
                res += num;
            } else if vec[num as usize] == 1 {
                res -= num;
            }
            vec[num as usize] += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::sum_of_unique(vec![1,2,3,2]));
    assert_eq!(0, Solution::sum_of_unique(vec![1,1,1,1,1]));
    assert_eq!(15, Solution::sum_of_unique(vec![1,2,3,4,5]));
    assert_eq!(1, Solution::sum_of_unique(vec![1]));
}