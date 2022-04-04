
struct NumArray {
    nums: Vec<i32>,
    tree: Vec<i32>
}

impl NumArray {

    fn low_bit(v: i32) -> i32 {
        v & -v
    }

    fn add(index: i32, val: i32, arr: &mut Vec<i32>) {
        let mut idx = index as usize;
        while idx < arr.len() {
            arr[idx] += val;
            idx += NumArray::low_bit(idx as i32) as usize;
        }
    }

    fn new(nums: Vec<i32>) -> Self {
        let mut tree = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            NumArray::add((i + 1) as i32, nums[i], &mut tree);
        }
        NumArray {
            nums,
            tree,
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        NumArray::add(index + 1, val - self.nums[index as usize], &mut self.tree);
        self.nums[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sum(right + 1) - self.prefix_sum(left)
    }

    fn prefix_sum(&self, index: i32) -> i32 {
        let mut idx = index;
        let mut sum = 0;
        while idx > 0 {
            sum += self.tree[idx as usize];
            idx -= NumArray::low_bit(idx);
        }
        sum
    }
}

#[test]
fn test() {
    let mut obj = NumArray::new(vec![1,3,5]);
    assert_eq!(9, obj.sum_range(0, 2));
    obj.update(1, 2);
    assert_eq!(8, obj.sum_range(0, 2));
}