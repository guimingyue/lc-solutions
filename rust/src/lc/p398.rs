use std::collections::HashMap;

/// #HashMap
struct Solution {
    value_idx_map: HashMap<i32, Vec<usize>>,
    value_cur_idx: HashMap<i32, usize>
}


impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut value_idx_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut value_cur_idx = HashMap::new();
        for (idx, v) in nums.iter().enumerate() {
            let vec = value_idx_map.entry(*v).or_default();
            vec.push(idx);
            value_cur_idx.insert(*v, 0);
        }
        Solution {
            value_idx_map,
            value_cur_idx
        }
    }

    fn pick(&mut self, target: i32) -> i32 {
        let idx = self.value_cur_idx.get_mut(&target).unwrap();
        let vec = self.value_idx_map.get(&target).unwrap();
        let res = vec[*idx];
        *idx = (*idx + 1) % vec.len();
        res as i32
    }
}

#[test]
fn test() {
    let mut solution = Solution::new(vec![1, 2, 3, 3, 3]);
    assert_eq!(2, solution.pick(3));
    assert_eq!(0, solution.pick(1));
    assert_eq!(3, solution.pick(3));
}