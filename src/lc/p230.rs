struct Solution {
    
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::lc::common::TreeNode::TreeNode;


/// Rc, RefCell, Vec
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        let mut p = root;
        let mut count = 0;
        while p.is_some() || !stack.is_empty() {
            if let Some(rc_node) = p {
                stack.push(Some(rc_node.clone()));
                p = rc_node.borrow().left.clone();
            } else {
                let q = stack.pop().unwrap();
                count += 1;
                if count == k {
                    return q.unwrap().borrow().val;
                }
                p = q.unwrap().borrow().right.clone();
            }
        }
        0
    }
}