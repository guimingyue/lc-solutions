use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let mut val = 0;
        // step[k] is the number of i == num[i] of the kth rotation.
        // for example: [2,3,1,4,0] is the array of nums
        // k == 0, the rotated array is [2,3,1,4,0], no element of the array is equals to the index, step[0] = 0,
        // k == 1, the rotated array is [3,1,4,0,2], 1 == num[1], so step[1] = 1;
        // k == 2, the rotated array is [1,4,0,2,3], step[2] = 0;
        // k == 3, the rotated array is [4,0,2,3,1], 2 == num[2] and 3 == num[3], so step[3] = 2;
        // k == 4, the rotated array is [0,2,3,1,4], 4 == num[4] , so step[1] = 1;
        let mut step = vec![0; nums.len()];
        for i in 0..nums.len() {
            if nums[i] <= i as i32 {
                step[i - nums[i] as usize] += 1;
                val += 1;
            } else {
                step[i + nums.len() - nums[i] as usize] += 1;
            }
        }

        let mut max_val = val;
        let mut res = 0;
        for i in 1..nums.len() {
            let v = val - step[i - 1] + 1;
            if v > max_val {
                max_val = v;
                res = i;
            }
            val = v;
        }
        res as i32
    }

    pub fn best_rotation_on2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_score = 0;
        let mut res = 0;
        for k in 0..n {
            let mut score = 0;
            for i in n - k..n {
                score += if nums[(i + k) % n] <= i as i32 {
                    1
                } else {
                    0
                };
            }
            for i in 0..n-k {
                score += if nums[(i + k) % n] <= i as i32 {
                    1
                } else {
                    0
                };
            }
            if score > max_score {
                max_score = score;
                res = k as i32;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::best_rotation(vec![2,3,1,4,0]));
    assert_eq!(0, Solution::best_rotation(vec![1,3,0,2,4]));
}