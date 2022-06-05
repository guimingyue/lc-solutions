struct SummaryRanges {
    list: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    fn new() -> Self {
        SummaryRanges {
            list: vec![]
        }
    }

    fn add_num(&mut self, val: i32) {
        let idx = self.list.binary_search(&val).unwrap_or_else(|x| x);
        self.list.insert(idx, val);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        if self.list.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut start = self.list[0];
        let mut end = start;
        let mut it = self.list.iter();
        while let Some(&v) = it.next() {
            if end == v || end + 1 == v {
                end = v;
            } else {
                res.push(vec![start, end]);
                start = v;
                end = v;
            }
        }
        res.push(vec![start, end]);
        res

    }
}

#[test]
fn test() {
    let mut obj = SummaryRanges::new();
    obj.add_num(6);
    assert_eq!(vec![vec![6,6]], obj.get_intervals());
    obj.add_num(6);
    assert_eq!(vec![vec![6,6]], obj.get_intervals());
    obj.add_num(0);
    assert_eq!(vec![vec![0,0], vec![6,6]], obj.get_intervals());
    obj.add_num(4);
    assert_eq!(vec![vec![0,0], vec![4,4],vec![6,6]], obj.get_intervals());
    obj.add_num(8);
    assert_eq!(vec![vec![0,0], vec![4,4],vec![6,6], vec![8,8]], obj.get_intervals());
    obj.add_num(7);
    assert_eq!(vec![vec![0,0], vec![4,4],vec![6,8]], obj.get_intervals());
    obj.add_num(6);
    assert_eq!(vec![vec![0,0], vec![4,4],vec![6,8]], obj.get_intervals());
    obj.add_num(5);
    assert_eq!(vec![vec![0,0], vec![4,8]], obj.get_intervals());
}
 