use crate::lc::Solution;
use std::collections::HashMap;

/// #HashMap
impl Solution{
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        fn dfs(max_choosable_integer: i32, usedNumber: usize, desired_total: i32, cur_total: i32, memo: &mut HashMap<usize, bool>) -> bool {
            if !memo.contains_key(&usedNumber) {
                let mut res = false;
                for i in 0..max_choosable_integer {
                    if usedNumber >> i & 1 == 0 {
                        if i + 1 + cur_total >= desired_total {
                            res = true;
                            break;
                        }
                        if !dfs(max_choosable_integer, usedNumber | (1 << i), desired_total, cur_total + i + 1, memo) {
                            res = true;
                            break;
                        }
                    }
                }
                memo.insert(usedNumber, res);
            }
            *(memo.get(&usedNumber).unwrap_or(&false))
        }

        if (1 + max_choosable_integer) * max_choosable_integer / 2 < desired_total {
            return false;
        }
        let mut memo = HashMap::new();
        dfs(max_choosable_integer, 0, desired_total, 0, &mut memo)
    }
}

#[test]
fn test() {
    assert!(!Solution::can_i_win(10, 40));
    assert!(Solution::can_i_win(10, 19));
    assert!(!Solution::can_i_win(10, 11));
    assert!(Solution::can_i_win(10, 0));
    assert!(Solution::can_i_win(10, 1));
}