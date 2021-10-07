use crate::lc::common::ListNode::ListNode;

pub struct Solution;


impl Solution {
    
    /*
      this solution refer to https://rustgym.com/leetcode/82 
     */
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut p = head;
        while let Some(node) = p.take() {
            let n_val = node.val;
            match stack.last() {
                Some(&(val, count))  if val == n_val => {
                    stack.pop();
                    stack.push((n_val, count+1));
                }
                _ => {
                    stack.push((n_val, 1))
                }
            }
            p = node.next;
        }
        let mut res = None;
        while let Some((val, count)) = stack.pop() {
            if count == 1 {
                res = Some(Box::new(ListNode{val, next: res}))
            }
        }
        res
    }
}


#[test]
fn test() {
    assert_eq!(vec![2],
       ListNode::to_vec(
           Solution::delete_duplicates(ListNode::new_from(vec![1, 1, 2]))
       )
    );
    assert_eq!(vec![2],
       ListNode::to_vec(
           Solution::delete_duplicates(ListNode::new_from(vec![1, 1, 2, 3, 3]))
       )
    );
    assert_eq!(Vec::<i32>::new(),
       ListNode::to_vec(
           Solution::delete_duplicates(ListNode::new_from(vec![1, 1]))
       )
    );
    assert_eq!(vec![1],
       ListNode::to_vec(
           Solution::delete_duplicates(ListNode::new_from(vec![1]))
       )
    );
    assert_eq!(Vec::<i32>::new(),
       ListNode::to_vec(
           Solution::delete_duplicates(ListNode::new_from(Vec::<i32>::new()))
       )
    );
    assert_eq!(vec![1, 2, 5],
       ListNode::to_vec(
           Solution::delete_duplicates(ListNode::new_from(vec![1, 2, 3, 3, 4, 4, 5]))
       )
    );

    assert_eq!(vec![2, 3],
       ListNode::to_vec(
           Solution::delete_duplicates(ListNode::new_from(vec![1, 1, 1, 2, 3]))
       )
    );
}