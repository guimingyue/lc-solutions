use crate::lc::Solution;

/// #Vec, BinaryHeap, Reverse, max
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let n = height.len();
        let mut heap = std::collections::BinaryHeap::new();
        let mut vis = vec![false; n];
        heap.push(std::cmp::Reverse((height[0], 0)));
        vis[0] = true;
        heap.push(std::cmp::Reverse((height[n - 1], n - 1)));
        vis[n-1] = true;
        let mut res = 0;
        let dirt:Vec<i32> = vec![-1, 1];
        while let Some(std::cmp::Reverse((h, i))) = heap.pop() {
            (0..2).map(|idx| i as i32 + dirt[idx])
                .filter(|&x| x >= 0 && (x as usize) < n)
                .map(|x| x as usize)
                .for_each(|x| {
                    if !vis[x] {
                        res += std::cmp::max(h - height[x], 0);
                        heap.push(std::cmp::Reverse((std::cmp::max(height[x], h), x as usize)));
                        vis[x] = true;
                    }
                })
        }
        res
    }
}



#[test]
fn test() {
    assert_eq!(6, Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
    assert_eq!(9, Solution::trap(vec![4,2,0,3,2,5]));
    assert_eq!(0, Solution::trap(vec![1]));
    assert_eq!(0, Solution::trap(vec![1, 2]));
    assert_eq!(0, Solution::trap(vec![1, 2, 3]));
    assert_eq!(0, Solution::trap(vec![3, 2, 1]));
}