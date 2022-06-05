use crate::lc::Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const IS_PRIME:[bool; 32] = [false, false, true, true, false, true, false, true, false, false, false,
            true, false, true, false, false, false, true, false, true, false, false, false, true, false, false,
            false, false, false, true, false, true];
        let mut res = 0;
        for i in left..=right {
            let count = i.count_ones();
            if IS_PRIME[count as usize] {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(23, Solution::count_prime_set_bits(842, 888));
    assert_eq!(4, Solution::count_prime_set_bits(6, 10));
    assert_eq!(5, Solution::count_prime_set_bits(10, 15));
}