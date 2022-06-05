use crate::lc::Solution;

/// #HashMap, unwrap_or, struct
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        #[derive(PartialEq, Eq, Default, Hash)]
        struct Point {
            x:i32,
            y:i32
        }
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (rectangles[0][0], rectangles[0][1], rectangles[0][2], rectangles[0][3]);
        let mut cnt = std::collections::HashMap::new();
        let mut area = 0;
        for p in rectangles.iter() {
            let rec_area = (p[2] - p[0]) * (p[3] - p[1]);
            area += rec_area;
            min_x = min_x.min(p[0]);
            min_y = min_y.min(p[1]);
            max_x = max_x.max(p[2]);
            max_y = max_y.max(p[3]);
            let p1 = Point{x: p[0], y: p[1]};
            let p2 = Point{x: p[2], y: p[1]};
            let p3 = Point{x: p[0], y: p[3]};
            let p4 = Point{x: p[2], y: p[3]};
            let v = cnt.entry(p1).or_insert(0);
            *v += 1;
            let v = cnt.entry(p2).or_insert(0);
            *v += 1;
            let v = cnt.entry(p3).or_insert(0);
            *v += 1;
            let v = cnt.entry(p4).or_insert(0);
            *v += 1;
        }

        let total_area = (max_x - min_x) * (max_y - min_y);
        let p_min_min = Point{x: min_x, y: min_y};
        let p_min_max = Point{x: min_x, y: max_y};
        let p_max_min = Point{x: max_x, y: min_y};
        let p_max_max = Point{x: max_x, y: max_y};
        if total_area != area || *(cnt.get(&p_min_min).unwrap_or(&0)) != 1
            || *(cnt.get(&p_min_max).unwrap_or(&0)) != 1 || *(cnt.get(&p_max_min).unwrap_or(&0)) != 1
            || *(cnt.get(&p_max_max).unwrap_or(&0)) != 1 {
            return false;
        }
        cnt.remove(&p_min_min);
        cnt.remove(&p_min_max);
        cnt.remove(&p_max_max);
        cnt.remove(&p_max_min);

        for num in cnt.values() {
            if *num != 2 && *num != 4 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(!Solution::is_rectangle_cover(vec![vec![0,0,1,1],vec![0,1,3,2],vec![1,0,2,2]]));
    assert!(Solution::is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![3,2,4,4],vec![1,3,2,4],vec![2,3,3,4]]));
    assert!(!Solution::is_rectangle_cover(vec![vec![1,1,2,3],vec![1,3,2,4],vec![3,1,4,2],vec![3,2,4,4]]));
    assert!(!Solution::is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![1,3,2,4],vec![3,2,4,4]]));
    assert!(!Solution::is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![1,3,2,4],vec![2,2,4,4]]));
}