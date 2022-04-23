use crate::lc::Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut vec = vec![];
        for (idx, ch) in s.chars().enumerate() {
            if ch == c {
                vec.push(idx);
            }
        }

        let mut res = vec![];
        let mut p = 0;
        for (idx, ch) in s.chars().enumerate() {
            let mut min = (idx as i32 - vec[p] as i32).abs();
            if idx == vec[p] && p + 1 < vec.len() {
                p += 1;
            }
            if p > 0 {
                let v = (vec[p - 1] as i32 - idx as i32).abs();
                if v < min {
                    min = v;
                }
            }
            res.push(min);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![2, 1, 0, 1], Solution::shortest_to_char("aaba".to_string(), 'b'));
    assert_eq!(vec![3,2,1,0,1,0,0,1,2,3,4], Solution::shortest_to_char("loveleetcod".to_string(), 'e'));
    assert_eq!(vec![3,2,1,0,1,0,0,1,2,2,1,0], Solution::shortest_to_char("loveleetcode".to_string(), 'e'));
    assert_eq!(vec![3,2,1,0], Solution::shortest_to_char("aaab".to_string(), 'b'));
}