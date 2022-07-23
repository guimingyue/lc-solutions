struct MyCalendarTwo {
    booked: Vec<Vec<i32>>,
    overlap: Vec<Vec<i32>>
}

impl MyCalendarTwo {

    fn new() -> Self {
        MyCalendarTwo {
            booked: vec![],
            overlap: vec![]
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for v in &self.overlap {
            if start < v[1] && end > v[0] {
                return false;
            }
        }
        for v in &self.booked {
            if start < v[1] && end > v[0] {
                self.overlap.push(vec![start.max(v[0]), end.min(v[1])]);
            }
        }
        self.booked.push(vec![start, end]);
        return true
    }
}

#[test]
fn test() {
    let mut obj = MyCalendarTwo::new();
    let ret_1: bool = obj.book(10, 20);
}