use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let (mut cnt0, mut cnt1, mut cnt2) = (0, 0, 0);
        for val in stones {
            let v = val % 3;
            if v == 0 {
                cnt0 += 1;
            } else if v == 1 {
                cnt1 += 1;
            } else {
                cnt2 += 1;
            }
        }
        if cnt0 % 2 == 0 {
            return cnt1 >= 1 && cnt2 >= 1;
        }
        cnt1 - cnt2 > 2 || cnt2 - cnt1 > 2
    }
}

#[test]
fn test() {
    assert!(Solution::stone_game_ix(vec![2,1]));
    assert!(!Solution::stone_game_ix(vec![2]));
    assert!(!Solution::stone_game_ix(vec![5,1,2,4,3]));
}