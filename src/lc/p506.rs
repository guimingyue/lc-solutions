use crate::lc::Solution;

/// #Vec, binary_search_by, sort_by, HashMap
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        map.insert(score.len() - 1, "Gold Medal");
        map.insert(score.len() - 2, "Silver Medal");
        map.insert(score.len() - 3, "Bronze Medal");
        let mut vec = vec![];
        for i in 0..score.len() {
            vec.push(i);
        }
        vec.sort_by(|a, b|  score[*a].cmp(&score[*b]));
        let mut res = vec![];
        for i in 0..score.len() {
            let v = score[i];
            let idx = vec.binary_search_by(|v_idx| score[*v_idx].cmp(&v)).unwrap_or_else(|x| x);
            if map.contains_key(&idx) {
                res.push(map.get(&idx).unwrap().to_string());
            } else {
                res.push((score.len() - idx).to_string());
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"], Solution::find_relative_ranks(vec![5,4,3,2,1]));
    assert_eq!(vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"], Solution::find_relative_ranks(vec![10,3,8,9,4]));
}