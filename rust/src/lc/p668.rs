
struct Solution {}

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (mut left, mut right) = (1, m * n);
        while left < right {
            let x = left + (right - left) / 2;
            let mut count = x / n * n;
            for i in x/n + 1..=m {
                count += x / i;
            }
            if count >= k {
                right = x;
            } else {
                left  = x + 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::find_kth_number(3, 3, 5));
    assert_eq!(6, Solution::find_kth_number(2, 3, 6));
    assert_eq!(6, Solution::find_kth_number(3, 4, 8));
    assert_eq!(6, Solution::find_kth_number(3, 4, 9));
}