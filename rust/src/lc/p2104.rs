use crate::lc::Solution;

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let (mut min_left, mut min_right, mut max_left, mut max_right) =
            (vec![-1; nums.len()], vec![-1; nums.len()], vec![-1; nums.len()], vec![-1; nums.len()]);
        let (mut min_stack, mut max_stack) = (vec![], vec![]);
        for i in 0..nums.len() {
            while !min_stack.is_empty() && nums[*min_stack.last().unwrap() as usize] > nums[i] {
                min_stack.pop();
            }
            min_left[i] = if min_stack.is_empty() { -1 } else { *min_stack.last().unwrap() };
            min_stack.push(i as i32);

            while !max_stack.is_empty() && nums[*max_stack.last().unwrap() as usize] < nums[i] {
                max_stack.pop();
            }
            max_left[i] = if max_stack.is_empty() { -1 } else { *max_stack.last().unwrap() };
            max_stack.push(i as i32);
        }
        min_stack.clear();
        max_stack.clear();
        for i in (0..nums.len()).rev() {
            while !min_stack.is_empty() && nums[*min_stack.last().unwrap() as usize] < nums[i] {
                min_stack.pop();
            }
            min_right[i] = if min_stack.is_empty() { nums.len() as i32 } else { *min_stack.last().unwrap() };
            min_stack.push(i as i32);

            while !max_stack.is_empty() && nums[*max_stack.last().unwrap() as usize] > nums[i] {
                max_stack.pop();
            }
            max_right[i] = if max_stack.is_empty() { -1 } else { *max_stack.last().unwrap() };
            max_stack.push(i as i32);
        }

        let (mut max, mut min) = (0 , 0);
        for i in 0..nums.len() {
            max += ((max_right[i] - i as i32) * (i as i32 - max_left[i]) * nums[i]) as i64;
            min += ((min_right[i] - i as i32) * (i as i32 - min_left[i]) * nums[i]) as i64;
        }

        max - min
    }

    pub fn sub_array_ranges_on2(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        for i in 0..nums.len() {
            let mut min = nums[i];
            let mut max = nums[i];
            for j in i + 1..nums.len() {
                if nums[j] > max {
                    max = nums[j];
                }
                if nums[j] < min {
                    min = nums[j];
                }
                res += (max - min) as i64;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::sub_array_ranges(vec![1,2,3]));
    assert_eq!(4, Solution::sub_array_ranges(vec![1,3,3]));
    assert_eq!(59, Solution::sub_array_ranges(vec![4,-2,-3,4,1]));
}