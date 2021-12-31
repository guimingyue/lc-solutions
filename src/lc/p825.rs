use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut ages = ages;
        ages.sort();
        let (mut l, mut r) = (0, 0);
        let mut res = 0;
        for i in 0..ages.len() {
            let age = ages[i];
            if age < 15 {
                continue;
            }
            while ages[l] <= age/2 + 7 {
                l += 1;
            }
            while r + 1 < ages.len() && ages[r+1] <= age {
                r += 1;
            }
            res += r - l;
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::num_friend_requests(vec![16]));
    assert_eq!(2, Solution::num_friend_requests(vec![16,16]));
    assert_eq!(2, Solution::num_friend_requests(vec![16,17,18]));
    assert_eq!(3, Solution::num_friend_requests(vec![20,30,100,110,120]));
}