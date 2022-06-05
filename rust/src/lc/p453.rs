struct Solution {}

/// #Vec
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut all_equal = true;
        for &n in nums.iter() {
            if min > n {
                min = n;
            }
            if min != i32::MAX && min != n {
                all_equal = false;
            }
        }
        if all_equal {
            return 0;
        }
        let mut res = 0;
        for n in nums {
            res += n - min;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::min_moves(vec![1, 2, 3]));
    assert_eq!(0, Solution::min_moves(vec![1, 1, 1]));
    assert_eq!(0, Solution::min_moves(vec![1]));
    assert_eq!(0, Solution::min_moves(Vec::<i32>::new()));
    assert_eq!(0, Solution::min_moves(vec![1, 1]));
    assert_eq!(2, Solution::min_moves(vec![1, 2, 2]));
    assert_eq!(1, Solution::min_moves(vec![1, 1, 2]));
}