use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut p4 = 3;
        let mut res = 0;
        while p4 < nums.len() {
            let mut p3 = p4 - 1;
            while p3 > 1 {
                let mut p2 = p3 - 1;
                while p2 > 0 {
                    let v = nums[p4] - nums[p3] - nums[p2];
                    let mut p1 = p2 - 1;
                    while p1 >= 0 {
                        if nums[p1] == v {
                            res += 1;
                        }
                        if p1 == 0 {
                            break;
                        }
                        p1 -= 1;
                    }
                    p2 -= 1;
                }
                p3 -= 1;
            }
            p4 += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::count_quadruplets(vec![1,2,3,6]));
    assert_eq!(0, Solution::count_quadruplets(vec![3,3,6,4,5]));
    assert_eq!(1, Solution::count_quadruplets(vec![3,3,6,4,5, 10]));
    assert_eq!(4, Solution::count_quadruplets(vec![1,1,1,3,5]));
    assert_eq!(1, Solution::count_quadruplets(vec![28,8,49,85,37,90,20,8]));
}