use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let (mut max_len, mut res) = (0, 0);
        for v in rectangles {
            let min = v[0].min(v[1]);
            if min == max_len {
                res += 1;
            } else if min > max_len {
                max_len = min;
                res = 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::count_good_rectangles(vec![vec![5,8],vec![3,9],vec![5,12],vec![16,5]]));
    assert_eq!(3, Solution::count_good_rectangles(vec![vec![2,3],vec![3,7],vec![4,3],vec![3,7]]));
    assert_eq!(1, Solution::count_good_rectangles(vec![vec![2,3]]));
}