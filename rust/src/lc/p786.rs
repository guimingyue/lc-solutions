use std::cmp::Ordering;
use crate::lc::Solution;

/// #sort_by, PartialOrd, Ord, PartialEq, BinaryHeap, Reverse
impl Solution {

    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {

        #[derive(Eq)]
        struct Tuple {
            i: usize,
            j: usize,
            d_i: i32,
            d_j: i32
        }

        impl PartialOrd for Tuple {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Tuple {
            fn cmp(&self, other: &Self) -> Ordering {
                let d1 = self.d_i * other.d_j;
                let d2 = self.d_j * other.d_i;
                d1.cmp(&d2)
            }
        }

        impl PartialEq for Tuple {
            fn eq(&self, other: &Self) -> bool {
                self.i == other.i && self.j == other.j
            }
        }

        let mut priority_queue = std::collections::BinaryHeap::new();
        for i in 1..arr.len() {
            priority_queue.push(std::cmp::Reverse(Tuple {
                i: 0,
                j: i,
                d_i: arr[0],
                d_j: arr[i]
            }));
        }
        let mut k = k;
        while let Some(std::cmp::Reverse(top)) = priority_queue.pop() {
            k -= 1;
            if k == 0 {
                return vec![top.d_i, top.d_j];
            }
            let new_idx = top.i + 1;
            if new_idx < top.j {
                priority_queue.push(std::cmp::Reverse(Tuple {
                    i: new_idx,
                    j: top.j,
                    d_i: arr[new_idx],
                    d_j: top.d_j
                }));
            }
        }
        vec![]
    }

    pub fn kth_smallest_prime_fraction_brute_force(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut coll = vec![];
        for i in 0..arr.len() {
            for j in i..arr.len() {
                coll.push((i, j));
            }
        }
        coll.sort_by(|a, b| (arr[a.0] * arr[b.1]).cmp(&(arr[a.1] * arr[b.0])));
        let idx = (k-1) as usize;
        vec![arr[coll[idx].0], arr[coll[idx].1]]
    }
}

#[test]
fn test() {
    assert_eq!(vec![2, 5], Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3));
    assert_eq!(vec![1, 7], Solution::kth_smallest_prime_fraction(vec![1, 7], 1));
}
