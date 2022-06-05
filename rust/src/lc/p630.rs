use crate::lc::Solution;

/// #Vec, sort_by, BinaryHeap
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by(|v1, v2| v1[1].cmp(&v2[1]));
        let mut heap = std::collections::BinaryHeap::new();
        let mut total = 0;
        for v in courses {
            let (ti, di) = (v[0], v[1]);
            if total + ti <= di {
                total += ti;
                heap.push(ti);
            } else if let Some(tj) = heap.peek() {
                if *tj > ti {
                    total -= *tj;
                    total += ti;
                    heap.pop();
                    heap.push(ti);
                }
            }
        }
        heap.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::schedule_course(vec![vec![100, 200], vec![200, 1300], vec![1000, 1250], vec![2000, 3200]]));
    assert_eq!(1, Solution::schedule_course(vec![vec![1,2]]));
    assert_eq!(0, Solution::schedule_course(vec![vec![3,2], vec![4,3]]));
    assert_eq!(4, Solution::schedule_course(vec![vec![7,17],vec![3,12],vec![10,20],vec![9,10],vec![5,20],vec![10,19],vec![4,18]]));
}