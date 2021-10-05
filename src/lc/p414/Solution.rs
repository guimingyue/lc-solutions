pub struct Solution {
    
}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut heap = vec![];
        for val in nums {
            for i in 0..3 {
                if let Some(v) = heap.get(i){
                    if val > *v {
                        heap.insert(i, val);
                        break;
                    } else if val == *v {
                        break;
                    }
                } else {
                    heap.push(val);
                    break;
                }
            }

            if heap.len() > 3 {
                heap.remove(3);
            }
        }
        if heap.len() == 3 {
            heap[2]
        } else {
            heap[0]
        }
    }
}