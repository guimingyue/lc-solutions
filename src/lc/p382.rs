use crate::lc::common::ListNode::ListNode;
use rand::Rng;

/// #Box, rand
struct Solution {
    head: Option<Box<ListNode>>
}

impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution {
            head
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let (mut i, mut res) = (1, 0);
        let mut p = self.head.as_ref();
        while let Some(n) = p {
            if rng.gen_range(0..i) == 0 {
                res = n.val;
            }
            i += 1;
            p = n.next.as_ref()
        }
        res
    }
}

#[test]
fn test() {
    let vec = vec![1,2,3];
    let s = Solution::new(ListNode::new_from(vec.clone()));
    let mut i = 0;
    while i < 100 {
        let v = s.get_random();
        assert!(vec.contains(&v));
        i += 1;
    }
}