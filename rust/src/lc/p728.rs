use crate::lc::Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        fn is_self_dividing_number(num: i32) -> bool {
            let mut v = num;
            while v > 0 {
                let m = v % 10;
                if m == 0 {
                    return false;
                }
                if num % m != 0 {
                    return false;
                }
                v /= 10;
            }
            true
        }
        let mut res = vec![];
        for i in left..=right {
            if is_self_dividing_number(i) {
                res.push(i);
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22], Solution::self_dividing_numbers(1, 22));
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Solution::self_dividing_numbers(1, 10));
    assert_eq!(vec![48,55,66,77], Solution::self_dividing_numbers(47, 85));
}