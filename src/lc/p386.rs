use crate::lc::Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut number = 1;
        for i in 0..n {
            res.push(number);
            if number * 10 <= n {
                number *= 10;
            } else {
                while number % 10 == 9 || number + 1 > n {
                    number /= 10;
                }
                number += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], Solution::lexical_order(13));
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Solution::lexical_order(9));
    assert_eq!(vec![1], Solution::lexical_order(1));
    assert_eq!(vec![1, 2], Solution::lexical_order(2));
    assert_eq!(vec![1,10,11,12,13,14,15,16,17,18,19,2,20,3,4,5,6,7,8,9], Solution::lexical_order(20));
}