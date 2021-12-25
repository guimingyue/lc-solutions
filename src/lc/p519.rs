use rand::Rng;

struct Solution {
    m: usize,
    n: usize,
    matrix: Vec<i32>
}

/// #Vec, rand
impl Solution {

    fn new(m: i32, n: i32) -> Self {
        let mut solution = Solution {
            m: m as usize,
            n: n as usize,
            matrix: vec![0; (m * n) as usize]
        };
        solution.reset();
        solution
    }

    fn flip(&mut self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let j = rng.gen_range(0..self.matrix.len());
        let val = self.matrix.remove(j);
        let line = val / (self.n as i32);
        vec![line as i32, val - (line * self.n  as i32)]
    }

    fn reset(&mut self) {
        self.matrix = vec![0; self.m * self.n];
        for i in 0..self.m {
            for j in 0..self.n {
                let idx = i * self.n + j;
                self.matrix[idx] = idx as i32;
            }
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(m, n);
 * let ret_1: Vec<i32> = obj.flip();
 * obj.reset();
 */
#[test]
fn test() {
    let mut obj = Solution::new(3, 1);
    let mut ret = obj.flip();
    obj.reset();
    let ret1 = obj.flip();
    let mut ret2 = obj.flip();
    let mut ret3 = obj.flip();
    obj.reset();
}