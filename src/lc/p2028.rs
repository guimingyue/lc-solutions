use crate::lc::Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len();
        let mut total = (m as i32 + n) * mean;
        for v in rolls {
            total -= v;
        }
        if total > 6 * n || total < n {
            return vec![];
        }
        let mut res = vec![total / n; n as usize];
        for i in 0..(total % n) as usize {
            res[i] += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![1,1], Solution::missing_rolls(vec![1], 1, 2));
    assert_eq!(Vec::<i32>::new(), Solution::missing_rolls(vec![6,3,4,3,5,3], 1, 6));
    assert_eq!(Vec::<i32>::new(), Solution::missing_rolls(vec![1,2,3,4], 6, 4));
    assert_eq!(vec![3,2,2,2], Solution::missing_rolls(vec![1,5,6], 3, 4));
    assert_eq!(vec![6,6], Solution::missing_rolls(vec![3,2,4,3], 4, 2));
}