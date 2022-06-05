struct MyQueue {
    push: Vec<i32>,
    pop: Vec<i32>
}

/// #Vec
impl MyQueue {

    fn new() -> Self {
        MyQueue {
            push: vec![],
            pop: vec![]
        }
    }

    fn push(&mut self, x: i32) {
        self.push.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.fill();
        self.pop.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.fill();
        *self.pop.last().unwrap()
    }

    fn fill(&mut self) {
        if self.pop.is_empty() {
            while let Some(v) = self.push.pop() {
                self.pop.push(v);
            }
        }
    }

    fn empty(&self) -> bool {
        self.push.is_empty() && self.pop.is_empty()
    }
}

#[test]
fn test() {
    let mut myQueue = MyQueue::new();
    myQueue.push(1);
    myQueue.push(2);
    assert_eq!(1, myQueue.peek());
    assert_eq!(1, myQueue.pop());
    assert!(!myQueue.empty());

}