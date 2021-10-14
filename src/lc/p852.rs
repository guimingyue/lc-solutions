struct Solution {}

/// Vec
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len() - 1;
        let mut mid= 0;
        while left < right {
            mid = (left + right) / 2;
            if arr[mid] > arr[mid-1] && arr[mid] > arr[mid + 1] {
                return mid as i32;
            } else if arr[mid] < arr[mid+1] {
                left = mid;
            } else {
                right = mid;
            }
        }
        return left as i32;
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 1, 0]));
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]));
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]));
    assert_eq!(2, Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]));
    assert_eq!(2, Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]));
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![24, 101, 100, 99, 79, 78, 67, 36, 26, 19]));
    assert_eq!(7, Solution::peak_index_in_mountain_array(vec![19, 26, 36, 67, 78, 79, 99, 100, 69, 24]));
    assert_eq!(8, Solution::peak_index_in_mountain_array(vec![19, 26, 36, 67, 78, 79, 99, 100, 101, 24]));
}