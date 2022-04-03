use std::collections::HashMap;
use crate::lc::Solution;

/// #HashMap
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for v in &arr {
            let num = map.entry(*v).or_default();
            *num += 1;
        }
        let mut visited = HashMap::new();
        if map.contains_key(&0) && map[&0] % 2 != 0 {
            return false;
        } else {
            visited.insert(0, true);
            map.remove(&0);
        }
        let mut vec = arr;
        vec.sort();
        for v in &vec {
            if visited.contains_key(v) {
                continue;
            }
            let mut another = 0;
            if *v < 0 {
                if *v % 2 != 0 {
                    return false;
                }
                another = *v / 2;
            } else {
                another = 2 * *v;
            }

            if !map.contains_key(&another) {
                return false;
            }
            let num_v = map.get_mut(v).unwrap();
            *num_v -= 1;
            if *num_v == 0 {
                map.remove(v);
                visited.insert(*v, true);
            }
            let num_a = map.get_mut(&another).unwrap();
            *num_a -= 1;
            if *num_a == 0 {
                visited.insert(another, true);
                map.remove(&another);
            }

        }
        map.is_empty()
    }
}

#[test]
fn test() {
    assert!(Solution::can_reorder_doubled(vec![2,4,0,0,8,1]));

    assert!(Solution::can_reorder_doubled(vec![1,2,1,-8,8,-4,4,-4,2,-2]));
    assert!(!Solution::can_reorder_doubled(vec![2,1,1,4,8,8]));
    assert!(Solution::can_reorder_doubled(vec![-2,-6,-3,4,-4,2]));
    assert!(!Solution::can_reorder_doubled(vec![2,-1,1,4,-2,4]));
    assert!(Solution::can_reorder_doubled(vec![2,1,2,1,1,1,2,2]));
    assert!(Solution::can_reorder_doubled(vec![2,4,8,1]));

    assert!(!Solution::can_reorder_doubled(vec![3,1,3,6]));
    assert!(!Solution::can_reorder_doubled(vec![2,1,2,6]));
    assert!(Solution::can_reorder_doubled(vec![4,-2,2,-4]));
}