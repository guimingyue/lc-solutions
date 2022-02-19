use crate::lc::Solution;

/// #Vec, Slice
impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut a = arr;
        let mut res = vec![];
        for n in (1..=a.len()).rev() {
            let mut max_index = 0;
            for i in 0..n {
                if a[i] > a[max_index] {
                    max_index = i;
                }
            }
            if max_index == n - 1 {
                continue;
            }
            if max_index > 0 {
                a[0..max_index + 1].reverse();
                res.push((max_index+1) as i32);
            }
            a[0..n].reverse();
            res.push(n as i32);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![3,4,2,3,2], Solution::pancake_sort(vec![3,2,4,1]));
    assert_eq!(Vec::<i32>::new(), Solution::pancake_sort(vec![1,2,3]));
    assert_eq!(vec![3], Solution::pancake_sort(vec![3,2,1]));
}