use crate::lc::Solution;

/// #i32
impl Solution {

    unsafe fn guessNumber(n: i32) -> i32 {

        /// this is a mock function for compiler
        unsafe fn guess(num: i32) -> i32 {
            -1
        }

        let (mut left, mut right) = (1, n);
        while left <= right {
            let mid = (right - left) / 2 + left;
            let res = guess(mid);
            if res == -1 {
                right = mid - 1;
            } else if res == 1 {
                left = mid + 1;
            } else {
                return mid;
            }
        }
        return n;
    }
}
