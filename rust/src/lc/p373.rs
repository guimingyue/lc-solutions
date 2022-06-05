use std::collections::BinaryHeap;
use crate::lc::Solution;

/// #Vec, BinaryHeap
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut ended = false;
        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                let sum = nums1[i] + nums2[j];
                if heap.len() < k as usize {
                    heap.push((sum, nums1[i], nums2[j]));
                } else {
                    if sum >= heap.peek().unwrap().0 {
                        if j == 0 {
                            ended = true;
                        }
                        break;
                    } else {
                        heap.pop();
                        heap.push((sum, nums1[i], nums2[j]));
                    }
                }
            }
            if ended {
                break;
            }
        }

        let mut res = vec![];
        while let Some ((sum, v1, v2)) = heap.pop() {
            res.push(vec![v1, v2]);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![vec![1,1],vec![1,1],vec![2,1],vec![1,2],vec![1,2],vec![2,2],vec![1,3],vec![1,3],vec![2,3]],
               Solution::k_smallest_pairs(vec![1,1,2], vec![1,2,3], 10));
    assert_eq!(vec![vec![1,2],vec![1,4],vec![1,6]], Solution::k_smallest_pairs(vec![1,7,11], vec![2,4,6], 3));
    assert_eq!(vec![vec![1,1],vec![1,1]], Solution::k_smallest_pairs(vec![1,1,2], vec![1,2,3], 2));
    assert_eq!(vec![vec![1,3],vec![2,3]], Solution::k_smallest_pairs(vec![1,2], vec![3], 3));
    assert_eq!(vec![vec![1,3]], Solution::k_smallest_pairs(vec![1], vec![3], 3));
}