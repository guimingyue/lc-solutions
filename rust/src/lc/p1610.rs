use crate::lc::Solution;

/// #Vec, sort_by, std::f64::consts::PI, partial_cmp, atan2
impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut same_cnt = 0;
        let mut polar_degree = vec![];
        for point in points {
            if point[0] == location[0] && point[1] == location[1] {
                same_cnt += 1;
            } else {
                let degree = ((point[1] - location[1]) as f64).atan2((point[0] - location[0]) as f64);
                polar_degree.push(degree);
            }
        }
        polar_degree.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..polar_degree.len() {
            polar_degree.push(polar_degree[i] + 2 as f64 * std::f64::consts::PI);
        }
        let degree = angle as f64 * std::f64::consts::PI / 180 as f64;
        let mut max_cnt = 0;
        for i in 0..polar_degree.len() {
            let target = polar_degree[i] + degree;
            fn upper_bound(vec: &Vec<f64>, target: f64) -> usize {
                let (mut left, mut right) = (0, vec.len() - 1);
                while left < right {
                    let mid = left + (right - left) / 2;
                    if vec[mid] > target {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }
                right
            }
            let idx = upper_bound(&polar_degree, target);
            max_cnt = max_cnt.max(idx - i);
        }
        (max_cnt + same_cnt) as i32
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::visible_points(vec![vec![2,1],vec![2,2],vec![3,3]], 90, vec![1,1]));
    assert_eq!(4, Solution::visible_points(vec![vec![2,1],vec![2,2],vec![3,4],vec![1,1]], 90, vec![1,1]));
    assert_eq!(1, Solution::visible_points(vec![vec![1,0],vec![2,1]], 13, vec![1,1]));
}