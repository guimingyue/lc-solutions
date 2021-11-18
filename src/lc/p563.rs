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
use crate::lc::Solution;
use crate::lc::common::TreeNode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

/// #Option, Rc, RefCell, abs
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn find(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> (i32, i32) {
            match root {
                Some(node_ref) => {
                    let node = node_ref.borrow();
                    let (left_total, left_tilt) = find(&node.left, res);
                    let (right_total, right_tilt) = find(&node.right, res);
                    let tilt = (left_total - right_total).abs();
                    *res += tilt;
                    (left_total + right_total + node.val, tilt)
                },
                None => (0, 0)
            }
        };
        let mut res = 0;
        find(&root, &mut res);
        res
    }
}

#[test]
fn test() {

}