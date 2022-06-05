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
use crate::lc::common::TreeNode::TreeNode;
use crate::lc::Solution;

/// #Rc, RefCell
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn convert(root: &Option<Rc<RefCell<TreeNode>>>, str: &mut String) {
            match root {
                Some(node_ref) => {
                    str.push('(');
                    let node = node_ref.borrow();
                    str.push_str(node.val.to_string().as_str());
                    if node.left.is_some() || node.right.is_some() {
                        if node.left.is_some() {
                            convert(&node.left, str);
                        } else {
                            str.push_str("()");
                        }

                        if node.right.is_some() {
                            convert(&node.right, str);
                        }
                    }
                    str.push(')');
                },
                _ => {}
            }
        }
        let mut str = String::new();
        convert(&root, &mut str);
        str.remove(0);
        str.remove(str.len()-1);
        str
    }
}

#[test]
fn test() {

}