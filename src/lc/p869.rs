use crate::lc::Solution;

/// #HashSet, BinaryHeap, Reverse
impl Solution {

    pub fn reordered_power_of2(n: i32) -> bool {
        if n & (n-1) == 0 {
            return true;
        }
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        let mut m = 1;
        for i in 1..31 {
            m = m << 1;
            set.insert(Solution::reorder(m));
        }
        set.contains(&Solution::reorder(n))
    }

    fn reorder(n: i32) -> i32{
        let mut heap = std::collections::BinaryHeap::new();
        let mut k = n;
        let mut zero_count = 0;
        while k > 0 {
            let mod_ = k % 10;
            if mod_ == 0 {
                zero_count += 1;
            } else {
                heap.push(std::cmp::Reverse(mod_));
            }
            k = k / 10;
        }
        let mut v = 0;
        while let Some(std::cmp::Reverse(val)) = heap.pop() {
            v = v * 10 + val;
        }
        if zero_count > 0 {
            v = v * 10;
            zero_count -= 1;
        }
        v
    }
}

#[test]
fn test() {
    assert!(Solution::reordered_power_of2(218));
    assert!(!Solution::reordered_power_of2(62));
    assert!(Solution::reordered_power_of2(1));
    assert!(Solution::reordered_power_of2(2));
    assert!(!Solution::reordered_power_of2(10));
    assert!(!Solution::reordered_power_of2(24));
    assert!(Solution::reordered_power_of2(61));
    assert!(Solution::reordered_power_of2(1204));
    assert!(!Solution::reordered_power_of2(1214));
    assert!(Solution::reordered_power_of2({2 as i32}.pow(30)));
    assert!(!Solution::reordered_power_of2({2 as i32}.pow(30) - 1));
}