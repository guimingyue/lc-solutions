// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use crate::lc::Solution;
use crate::lc::common::TreeNode::TreeNode;

/// #Rc, RefCell, Option
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn find(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node_ref) = root {
                let temp = node_ref.clone();
                let node = node_ref.borrow();
                if node.val == val {
                    Some(temp)
                } else {
                    if node.val > val {
                        find(&node.left, val)
                    } else {
                        find(&node.right, val)
                    }
                }
            } else {
                None
            }
        };

        find(&root, val)
    }
}