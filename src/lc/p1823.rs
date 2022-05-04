use crate::lc::Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut vec = vec![];
        for i in 1..=n {
            vec.push(i);
        }
        let mut i = 0;
        while vec.len() > 1 {
            let idx = (i + k - 1) as usize % vec.len();
            vec.remove(idx);
            i = idx as i32;
        }
        vec[0]
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::find_the_winner(5, 2));
    assert_eq!(1, Solution::find_the_winner(6, 5));
    assert_eq!(121, Solution::find_the_winner(500, 499));
}