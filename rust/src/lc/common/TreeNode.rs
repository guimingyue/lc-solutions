use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
        val,
        left: None,
        right: None
        }
    }
    
    pub fn new_tree(tree: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if tree.is_empty() {
            return None;
        }
        let mut quque = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        for (i, item) in tree.iter().enumerate() {
            if let Some(v) = item {
                if quque.is_empty() {
                    quque.push_back(Rc::new(RefCell::new(TreeNode::new(*v))));
                } else {
                    let v = quque.pop_front().unwrap();
                }
            } else {
                
            }
        }
        None
    }
    
    fn to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        vec![]
    }
}

#[test]
fn test() {
    
}