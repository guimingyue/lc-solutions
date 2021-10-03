use crate::lc::common::ListNode::ListNode;

pub struct Solution {
    
}

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut prev = None;
        let mut p = head;
        while let Some(mut node) = p.take() {
            let next = node.next.take();
            node.next = prev.take();
            p = next;
            prev = Some(node);
        }
        prev
    }
}