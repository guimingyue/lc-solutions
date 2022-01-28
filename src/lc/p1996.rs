use crate::lc::Solution;

/// #Vec, sort_by
impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut prop = properties;
        prop.sort_by(|a, b|
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            });
        let mut res = 0;
        let mut max_def = 0;
        for b in prop {
            if b[1] < max_def {
                res += 1;
            } else {
                max_def = b[1];
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::number_of_weak_characters(vec![vec![5,5],vec![6,3],vec![3,6]]));
    assert_eq!(1, Solution::number_of_weak_characters(vec![vec![2,2],vec![3,3]]));
    assert_eq!(2, Solution::number_of_weak_characters(vec![vec![2,2],vec![3,3], vec![4,4]]));
    assert_eq!(1, Solution::number_of_weak_characters(vec![vec![1,5],vec![10,4],vec![4,3]]));
}