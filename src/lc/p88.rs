use crate::lc::Solution;

/// #Vec
impl Solution {

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = (m-1) as usize;
        if m > 0 {
            while i >= 0 {
                nums1.swap(i, i + (n as usize));
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        let (mut n1, mut n2) = (nums1.len()- m as usize, 0 as usize);
        i = 0;
        if m > 0 {
            while n1 < nums1.len() && n2 < (n as usize) {
                if nums1[n1] <= nums2[n2] {
                    let v = nums1[n1];
                    nums1[i] = nums1[n1];
                    n1 += 1;
                } else {
                    nums1[i] = nums2[n2];
                    n2 += 1;
                }
                i += 1;
            }
        }
        while n2 < (n as usize) {
            nums1[i] = nums2[n2];
            n2 += 1;
            i += 1;
        }
    }
}

#[test]
fn test() {
    test_fn(&mut vec![1,2,3,0,0,0], 3, &mut vec![2,5,6], 3, &vec![1,2,2,3,5,6]);
    test_fn(&mut vec![4,0,0,0,0,0], 1, &mut vec![1,2,3,5,6], 5, &vec![1,2,3,4,5,6]);
    test_fn(&mut vec![0], 0, &mut vec![1], 1, &vec![1]);
    test_fn(&mut vec![0,0], 0, &mut vec![1,2], 2, &vec![1,2]);
    test_fn(&mut vec![1], 1, &mut Vec::<i32>::new(), 0, &vec![1]);
    test_fn(&mut vec![1,2], 2, &mut Vec::<i32>::new(), 0, &vec![1,2]);
}

fn test_fn(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32, expect: &Vec<i32>) {
    Solution::merge(nums1, m, nums2, n);
    assert_eq!(nums1, expect);
}