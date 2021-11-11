use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut pre = vec![1];
        for row in 1..=row_index {
            let mut next = vec![1];
            for i in 0..pre.len() - 1 {
                next.push(pre[i] + pre[i+1]);
            }
            next.push(1);
            pre = next;
        }
        pre
    }
}

#[test]
fn test() {
    assert_eq!(vec![1,3,3,1], Solution::get_row(3));
    assert_eq!(vec![1], Solution::get_row(0));
    assert_eq!(vec![1, 1], Solution::get_row(1));
}