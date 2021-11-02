use crate::lc::Solution;

impl Solution {

    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut p0, mut p2) = (0, nums.len()-1);
        let mut i = 0;
        while i <= p2 {
            while i <= p2 && nums[i] == 2 {
                let temp = nums[i];
                nums[i] = nums[p2];
                nums[p2] = temp;
                if p2 == 0 {
                    break;
                }
                p2 -= 1;
            }
            if nums[i] == 0 {
                let temp = nums[p0];
                nums[p0] = nums[i];
                nums[i] = temp;
                p0 += 1;
            }
            i += 1;
        }

    }

    pub fn sort_colors_(nums: &mut Vec<i32>) {
        let (mut p0, mut p1) = (0, 0);
        for i in 0..nums.len() {
            if nums[i] == 1 {
                let temp = nums[p1];
                nums[p1] = nums[i];
                nums[i] = temp;
                p1 += 1;
            } else if nums[i] == 0 {
                let temp = nums[p0];
                nums[p0] = nums[i];
                nums[i] = temp;
                if p0 < p1 {
                    let temp = nums[p1];
                    nums[p1] = nums[i];
                    nums[i] = temp;
                };
                p0 += 1;
                p1 += 1;
            }
        }
    }
}

#[test]
fn test() {
    test_fn(vec![2,1,0,2,0,1], vec![0,0,1,1,2,2]);
    test_fn(vec![2,0,2,1,1,0], vec![0,0,1,1,2,2]);
    test_fn(vec![2,0,1], vec![0,1,2]);
    test_fn(vec![0], vec![0]);
    test_fn(vec![1], vec![1]);
    test_fn(vec![2], vec![2]);
}

fn test_fn(mut vec: Vec<i32>, expect: Vec<i32>) {
    Solution::sort_colors(&mut vec);
    assert_eq!(expect, vec);
}