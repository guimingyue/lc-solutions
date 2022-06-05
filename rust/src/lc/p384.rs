
struct Solution {
    nums: Vec<i32>,
    shuffle: Vec<i32>
}

/// #Vec, rand, clone
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Solution {
            shuffle: nums.clone(),
            nums
        }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.shuffle = self.nums.clone();
        self.shuffle.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for i in 0..self.nums.len() {
            let j = i + rng.gen_range(0..self.nums.len() - i);
            self.shuffle.swap(i, j);
        }
        self.shuffle.clone()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let mut obj = Solution::new(nums.clone());
    let ret_1: Vec<i32> = obj.reset();
    assert_eq!(nums, ret_1);
    let ret_2: Vec<i32> = obj.shuffle();
}