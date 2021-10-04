/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  
  pub fn new_from(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut nums = nums;
    nums.reverse();
    let mut p = None;
    for (i, item) in nums.iter().enumerate() {
      if i == nums.len() - 1 {
        let mut head = ListNode::new(*item);
        head.next = p;
        return Some(Box::new(head));
      } else {
        let mut node = ListNode::new(*item);
        node.next = p;
        p = Some(Box::new(node));
      }
    }
    None
  }
  
  pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = vec![];
    let mut p = head;
    while let Some(t) = p {
      vec.push(t.val);
      p = t.next;
    }
    vec
  }
}