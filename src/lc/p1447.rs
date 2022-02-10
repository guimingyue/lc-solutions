use crate::lc::Solution;

/// #format!
impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn exist_common_divisor(m: i32, n: i32) -> bool {
            for k in 2..=m {
                if m % k == 0 && n % k == 0 {
                    return true;
                }
            }
            false
        }
        let mut res = vec![];
        for i in 2..=n {
            for j in 1..i {
                if j > 1 && exist_common_divisor(j, i) {
                    continue;
                }
                res.push(format!("{}/{}", j, i));
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec!["1/2"], Solution::simplified_fractions(2));
    assert_eq!(vec!["1/2","1/3","2/3"], Solution::simplified_fractions(3));
    assert_eq!(vec!["1/2","1/3","2/3","1/4","3/4"], Solution::simplified_fractions(4));
}
