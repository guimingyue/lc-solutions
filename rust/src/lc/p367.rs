use crate::lc::Solution;

/// #pow
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }
        let mut left:i32 = 1;
        let mut right:i32 = 46340;
        while left <= right {
            let mid = (left + right) / 2;
            let pow = mid.pow(2);
            if pow == num {
                return true;
            } else if pow > num {
                right = mid-1;
            } else {
                left = mid+1;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::is_perfect_square(1));
    assert!(!Solution::is_perfect_square(2));
    assert!(!Solution::is_perfect_square(3));
    assert!(Solution::is_perfect_square(4));
    assert!(!Solution::is_perfect_square(5));
    assert!(!Solution::is_perfect_square(6));
    assert!(!Solution::is_perfect_square(7));
    assert!(!Solution::is_perfect_square(8));
    assert!(Solution::is_perfect_square(9));
    assert!(!Solution::is_perfect_square(10));
    assert!(!Solution::is_perfect_square(11));
    assert!(!Solution::is_perfect_square(12));
    assert!(!Solution::is_perfect_square(13));
    assert!(!Solution::is_perfect_square(14));
    assert!(!Solution::is_perfect_square(15));
    assert!(Solution::is_perfect_square(16));
    assert!(Solution::is_perfect_square(25));
    assert!(Solution::is_perfect_square(36));
    assert!(Solution::is_perfect_square(49));
    assert!(Solution::is_perfect_square(64));
    assert!(!Solution::is_perfect_square(51));
    assert!(Solution::is_perfect_square(46340 * 46340));

}