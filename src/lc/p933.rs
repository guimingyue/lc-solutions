use std::collections::LinkedList;

/// LinkedList
struct RecentCounter {
    queue: LinkedList<i32>,
    sum: i32
}


impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            queue: LinkedList::new(),
            sum: 0
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        self.sum += 1;
        while *(self.queue.front().unwrap()) < t - 3000 {
            self.queue.pop_front();
            self.sum -= 1;
        }
        self.sum
    }
}

#[test]
fn test() {
    let mut recentCounter = RecentCounter::new();
    assert_eq!(1, recentCounter.ping(1));
    assert_eq!(2, recentCounter.ping(100));
    assert_eq!(3, recentCounter.ping(3001));
    assert_eq!(3, recentCounter.ping(3002));
}