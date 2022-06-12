use std::collections::BTreeMap;

struct MyCalendarThree {
    cnt: BTreeMap<i32, i32>
}


impl MyCalendarThree {

    fn new() -> Self {
        MyCalendarThree {
            cnt: BTreeMap::new()
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        let start_entry = self.cnt.entry(start).or_default();
        *start_entry += 1;
        let end_entry = self.cnt.entry(end).or_default();
        *end_entry -= 1;
        let mut max_book = 0;
        let mut ans = 0;
        self.cnt.iter().for_each(|(k, v)| {
            max_book += *v;
            ans = ans.max(max_book)
        });
        ans
    }
}

#[test]
fn test() {
    let mut obj = MyCalendarThree::new();
    assert_eq!(1, obj.book(10, 20));
    assert_eq!(1, obj.book(50, 60));
    assert_eq!(2, obj.book(10, 40));
    assert_eq!(3, obj.book(5, 15));
    assert_eq!(3, obj.book(5, 10));
    assert_eq!(3, obj.book(25, 55));
}