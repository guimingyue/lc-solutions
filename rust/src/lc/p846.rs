use std::collections::{BTreeMap, VecDeque};
use std::iter::FromIterator;
use crate::lc::Solution;

/// #BTreeMap, VecDeque, FromIterator
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut tree_map = BTreeMap::<i32, i32>::new();
        for h in hand {
            let v = tree_map.entry(h).or_default();
            *v += 1;
        }
        let mut seq_queue = VecDeque::from_iter(tree_map);
        let mut queue = vec![];
        while !seq_queue.is_empty() {
            let mut pre_v = -1;
            for i in 0..group_size {
                let first = seq_queue.pop_front();
                if first.is_none() {
                    return false;
                }
                let first_v = first.unwrap();
                if pre_v > 0 {
                    if first_v.0 - pre_v != 1 {
                        return false;
                    } else {
                        pre_v = first.unwrap().0;
                    }
                }
                pre_v = first_v.0;
                if first_v.1 > 1 {
                    queue.push((first_v.0, first_v.1 - 1));
                }
            }
            while let Some(v) = queue.pop() {
                seq_queue.push_front(v);
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::is_n_straight_hand(vec![1,2,3,4], 4));
    assert!(Solution::is_n_straight_hand(vec![1,2,3,6,2,3,4,7,8], 3));
    assert!(!Solution::is_n_straight_hand(vec![1,2,3,4,5], 4));
}