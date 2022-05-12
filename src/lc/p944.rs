use crate::lc::Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let col = strs[0].len();
        let mut res = 0;
        for j in 0..col {
            let mut last = strs[0].as_bytes()[j];
            for i in 1..strs.len() {
                let cur  = strs[i].as_bytes()[j];
                if last > cur {
                    res += 1;
                    break
                }
                last = cur;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::min_deletion_size(vec!["rrjk".to_string(),"furt".to_string(),"guzm".to_string()]));
    assert_eq!(1, Solution::min_deletion_size(vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()]));
    assert_eq!(0, Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()]));
    assert_eq!(3, Solution::min_deletion_size(vec!["zyx".to_string(),"wvu".to_string(),"tsr".to_string()]));
}