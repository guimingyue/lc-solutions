use crate::lc::common::ListNode::ListNode;

pub struct Solution {
    
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut newHead = Some(Box::new(ListNode{val: -1, next: head}));
        let mut length = 0;
        {
            let mut p = newHead.as_ref();
            while p.unwrap().next.is_some(){
                length += 1;
                p = p.unwrap().next.as_ref();
            }
        }
        let idx = length - n;
        {
            let mut p = newHead.as_mut();
            for i in 0..idx {
                p = p.unwrap().next.as_mut();
            }
            // p.next = p.next.next;
            p.as_mut().unwrap().next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        }

        newHead.unwrap().next
    }
}