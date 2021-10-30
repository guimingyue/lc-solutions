struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = [0;32];
        for v in nums {
            for i in 0..32 {
                a[i] += (v >> i) & 1;
            }
        }
        let mut res = 0;
        for i in 0..32 {
            if a[i] % 3 != 0 {
                res += 1 << i;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::single_number(vec![2,2,3,2]));
    assert_eq!(-3, Solution::single_number(vec![2,2,-3,2]));
    assert_eq!(3, Solution::single_number(vec![-2,-2,3,-2]));
    assert_eq!(99, Solution::single_number(vec![0,1,0,1,0,1,99]));
    assert_eq!(99, Solution::single_number(vec![0,-1,0,-1,0,-1,99]));
}