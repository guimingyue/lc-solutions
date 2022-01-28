use std::collections::HashMap;

/// #HashMap
struct DetectSquares {
    points_count: HashMap<Vec<i32>, u32>,
    x_points: HashMap<i32, Vec<i32>>,
    y_points: HashMap<i32, Vec<i32>>
}

impl DetectSquares {

    fn new() -> Self {
        DetectSquares {
            points_count: HashMap::new(),
            x_points: HashMap::new(),
            y_points: HashMap::new()
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let y_vec = self.x_points.entry(point[0]).or_default();
        y_vec.push(point[1]);
        let x_vec = self.y_points.entry(point[1]).or_default();
        x_vec.push(point[0]);
        let count = self.points_count.entry(point).or_default();
        *count += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut res:u32 = 0;
        if let Some(x_vec) = self.y_points.get(&point[1]) {
            if let Some(y_vec) = self.x_points.get(&point[0]) {
                for x in x_vec {
                    for y in y_vec {
                        let key = vec![*x, *y];
                        match self.points_count.get(&key) {
                            Some(count) if (*x - point[0]).abs() == (*y - point[1]).abs() && *x - point[0] != 0 => res += *count,
                            _ => {}
                        }
                    }
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let mut detectSquares = DetectSquares::new();
    detectSquares.add(vec![3, 10]);
    detectSquares.add(vec![11, 2]);
    detectSquares.add(vec![3, 2]);
    assert_eq!(1, detectSquares.count(vec![11, 10]));
    assert_eq!(0, detectSquares.count(vec![14, 8]));
    detectSquares.add(vec![11, 2]);
    assert_eq!(2, detectSquares.count(vec![11, 10]));

    let mut detectSquares = DetectSquares::new();
    detectSquares.add(vec![5,10]);
    detectSquares.add(vec![10,5]);
    detectSquares.add(vec![10,10]);
    assert_eq!(1, detectSquares.count(vec![5,5]));
    detectSquares.add(vec![3,0]);
    detectSquares.add(vec![8,0]);
    detectSquares.add(vec![8,5]);
    assert_eq!(1, detectSquares.count(vec![3,5]));
    detectSquares.add(vec![9,0]);
    detectSquares.add(vec![9,8]);
    detectSquares.add(vec![1,8]);
    assert_eq!(1, detectSquares.count(vec![1,0]));
    detectSquares.add(vec![0,0]);
    detectSquares.add(vec![8,0]);
    detectSquares.add(vec![8,8]);
    assert_eq!(2, detectSquares.count(vec![0,8]));
}