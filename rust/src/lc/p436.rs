use crate::lc::Solution;

/// #sort_by
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        fn binary_search(vec: &Vec<(i32, usize)>, v: i32) -> i32 {
            if vec[vec.len() - 1].0 < v {
                return -1;
            }
            let (mut left, mut right) = (0, vec.len()-1);
            while left < right {
                let mid = left + (right - left) / 2;
                if vec[mid].0 < v {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            vec[left].1 as i32
        }
        let mut vec = vec![];
        for (idx, v) in intervals.iter().enumerate() {
            vec.push((v[0], idx));
        }
        vec.sort_by(|a, b|
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        );
        let mut res = vec![-1; intervals.len()];
        for i in 0..intervals.len() {
            res[i] = binary_search(&vec, intervals[i][1]);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![-1, 2, -1], Solution::find_right_interval(vec![vec![1,4],vec![2,3],vec![3,4]]));
    assert_eq!(vec![-1, 0, 1], Solution::find_right_interval(vec![vec![3,4],vec![2,3],vec![1,2]]));
    assert_eq!(vec![-1], Solution::find_right_interval(vec![vec![1,2]]));
}