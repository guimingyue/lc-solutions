use crate::lc::Solution;

/// #Vec, ^, >>, <<
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut axb = 0;
        let mut iter = nums.iter();
        while let Some(&val) = iter.next() {
            axb = axb ^ val;
        }

        let idx = Solution::lower_1_bit_idx(axb);

        let mask = 1 << idx;
        let mut a = 0;
        let mut b = 0;
        for val in nums {
            if val & mask > 0 {
                a = a ^ val
            } else {
                b = b ^ val;
            }
        }

        vec![a, b]
    }

    fn lower_1_bit_idx(num: i32) -> i32{
        for i in 0..31 {
            if (num >> i) & 1 == 1 {
                return i;
            }
        }
        return 31;
    }
}

#[test]
fn test() {
    assert_eq!(vec![3, 5], Solution::single_number(vec![1,2,1,3,2,5]));
    assert_eq!(vec![-1, 0], Solution::single_number(vec![-1,0]));
    assert_eq!(vec![1, 0], Solution::single_number(vec![0,1]));
    assert_eq!(vec![-1, 1], Solution::single_number(vec![-1,1]));
    assert_eq!(vec![-1, 1], Solution::single_number(vec![2,-1,2,1]));
    assert_eq!(vec![-1, 1], Solution::single_number(vec![-2,-1,-2,1]));
    assert_eq!(vec![-5, -3], Solution::single_number(vec![-2,-1,-2,-1,-3, -5]));
    assert_eq!(vec![-5, -3], Solution::single_number(vec![i32::MIN,-1,i32::MIN,-1,-3, -5]));
    assert_eq!(vec![-5, -3], Solution::single_number(vec![i32::MIN,-1,i32::MIN,-1,-3, i32::MAX, -5, i32::MAX]));
}