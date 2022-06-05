use crate::lc::common::ListNode::ListNode;

pub struct Solution {
    
}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut head = head;
        let mut p = head.as_mut();
        while p.is_some() {
            let next = p.as_ref().unwrap().next.as_ref();
            if next.is_none() {
                break;
            }
            if p.as_ref().unwrap().val == next.as_ref().unwrap().val {
                let mut mut_next = p.as_mut().unwrap().next.as_mut();
                p.as_mut().unwrap().next = mut_next.as_mut().unwrap().next.take();
            } else {
                p = p.unwrap().next.as_mut();
            }
        }

        head
    }
}