use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::lc::Solution;

/// #Vec, Reverse
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut day = 0;
        let mut heap = BinaryHeap::new();
        let n = apples.len();
        while day < n || !heap.is_empty() {
            if day < n {
                heap.push(Reverse((days[day] + day as i32, apples[day])));
            }
            while let Some(Reverse(mut top)) = heap.pop() {
                if top.0 - day as i32  <= 0 || top.1 <= 0 {
                    continue;
                } else {
                    top.1 -= 1;
                    res += 1;
                    if top.1 > 0 {
                        heap.push(Reverse((top.0, top.1)));
                    }
                    break;
                }
            }
            day += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(5, Solution::eaten_apples(vec![9,2], vec![3,5]));
    assert_eq!(7, Solution::eaten_apples(vec![1,2,3,5,2], vec![3,2,1,4,2]));
    assert_eq!(5, Solution::eaten_apples(vec![3,0,0,0,0,2], vec![3,0,0,0,0,2]));
    assert_eq!(3, Solution::eaten_apples(vec![3,0,0,0,0,0], vec![3,0,0,0,0,0]));
    assert_eq!(2, Solution::eaten_apples(vec![2], vec![3]));
    assert_eq!(1, Solution::eaten_apples(vec![1], vec![1]));
    assert_eq!(1, Solution::eaten_apples(vec![2], vec![1]));
}