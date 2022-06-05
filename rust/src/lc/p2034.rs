use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// #BinaryHeap, HashMap, Reverse
struct StockPrice {
    max_timestamp: i32,
    map: HashMap<i32, i32>,
    pq_max: BinaryHeap<(i32, i32)>,
    pq_min: BinaryHeap<Reverse<(i32, i32)>>
}


impl StockPrice {

    fn new() -> Self {
        StockPrice {
            max_timestamp: 0,
            map: HashMap::new(),
            pq_max: BinaryHeap::new(),
            pq_min: BinaryHeap::new()
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.max_timestamp = self.max_timestamp.max(timestamp);
        self.map.insert(timestamp, price);
        self.pq_max.push((price, timestamp));
        self.pq_min.push(Reverse((price, timestamp)));
    }

    fn current(&self) -> i32 {
        *self.map.get(&self.max_timestamp).unwrap()
    }

    fn maximum(&mut self) -> i32 {
        while let Some((price, timestamp)) = self.pq_max.peek() {
            if *self.map.get(timestamp).unwrap() == *price {
                return *price;
            }
            self.pq_max.pop();
        }
        return -1;
    }

    fn minimum(&mut self) -> i32 {
        while let Some(Reverse((price, timestamp))) = self.pq_min.peek() {
            if *self.map.get(timestamp).unwrap() == *price {
                return *price;
            }
            self.pq_min.pop();
        }
        return -1;
    }
}


#[test]
fn test() {
    let mut obj = StockPrice::new();
    obj.update(1, 2);
    let ret_2: i32 = obj.current();
    let ret_3: i32 = obj.maximum();
    let ret_4: i32 = obj.minimum();
}
