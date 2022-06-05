use crate::lc::Solution;

/// Vec
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        fn gray_code(n: i32, vec: &mut Vec<i32>) {
            if n == 1 {
                vec.push(0);
                vec.push(1);
                return;
            }
            gray_code(n - 1, vec);
            for i in (0..vec.len()).rev() {
                vec.push(vec[i] ^ 1 << (n-1));
            }
        }
        let mut res = vec![];
        gray_code(n, &mut res);
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![0, 1], Solution::gray_code(1));
    assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2));
    assert_eq!(vec![0, 1, 3, 2, 6, 7, 5, 4], Solution::gray_code(3));
}