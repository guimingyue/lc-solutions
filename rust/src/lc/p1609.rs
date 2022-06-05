use crate::lc::Solution;
use crate::lc::common::TreeNode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

/// #Rc, RefCell, Vec
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check_val(first: &Rc<RefCell<TreeNode>>, level: i32) -> bool {
            (level % 2 == 0 && first.borrow().val % 2 == 1) ||
                (level % 2 == 1 && first.borrow().val % 2 == 0)
        }
        fn compare(before: &Rc<RefCell<TreeNode>>, next: &Rc<RefCell<TreeNode>>, level: i32) -> bool {
            (level % 2 == 0 && before.borrow().val < next.borrow().val) ||
                (level % 2 == 1 && before.borrow().val > next.borrow().val)
        }
        fn addChild(first: &Rc<RefCell<TreeNode>>, next: &mut Vec<Rc<RefCell<TreeNode>>>) {
            if let Some(rc) = first.borrow().left.clone() {
                next.push(rc);
            }
            if let Some(rc) = first.borrow().right.clone() {
                next.push(rc);
            }
        }
        let mut pre = vec![];
        let mut next = vec![];
        pre.push(root.unwrap().clone());
        let mut level = -1;
        while !pre.is_empty() {
            level += 1;
            let mut first = pre.remove(0);
            if !check_val(&first, level) {
                return false;
            }
            addChild(&first, &mut next);
            while !pre.is_empty() {
                let rc = pre.remove(0);
                if !compare(&first, &rc, level) {
                    return false;
                }
                first.swap(&rc);
                if !check_val(&first, level) {
                    return false;
                }
                addChild(&first, &mut next);

            }
            pre.append(&mut next);
            next.clear();
        }
        true
    }
}

#[test]
fn test() {

}