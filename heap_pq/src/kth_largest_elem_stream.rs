use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    nums: BinaryHeap<Reverse<i32>>,
    size: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let size = k as usize;

        let mut obj = Self {
            // +1 because we are protecting from dynamic allocation while we push one
            // extra element before popping.
            nums: BinaryHeap::with_capacity(size + 1),
            size,
        };
        for num in nums {
            obj.add(num);
        }
        obj
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(Reverse(val));
        if self.nums.len() > self.size {
            self.nums.pop();
        }
        self.nums.peek().unwrap().0
    }
}
