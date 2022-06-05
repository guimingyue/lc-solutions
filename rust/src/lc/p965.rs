use std::rc::Rc;
use std::cell::RefCell;
use crate::lc::common::TreeNode::TreeNode;
use crate::lc::Solution;

/// #Rc, RefCell
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_unival_node(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut i32) -> bool {
            match root {
                Some(rc) => {
                    let rcv = rc.borrow();
                    if *v == -1 {
                        *v = rcv.val;
                    }
                    rcv.val == *v && is_unival_node(&rcv.left, v) && is_unival_node(&rcv.right, v)
                },
                None => true
            }
        }
        let mut v = -1;
        is_unival_node(&root, &mut v)
    }
}
