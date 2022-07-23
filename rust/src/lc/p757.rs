use crate::lc::Solution;

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b|
            if a[0] != b[0] {
                return a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            });
        let (n, m) = (intervals.len(), 2);
        let mut vals = vec![vec![]; n];
        let mut ans = 0;
        for i in (0..n).rev() {
            let (mut j, mut k) = (intervals[i][0], vals[i].len());
            while k < m {
                ans += 1;
                let mut p = (i - 1) as i32;
                while p >= 0 && intervals[p as usize][1] >= j {
                    vals[p as usize].push(j);
                    p -= 1;
                }
                k += 1;
                j += 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(5, Solution::schedule_course(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]));
    assert_eq!(3, Solution::schedule_course(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]));

}