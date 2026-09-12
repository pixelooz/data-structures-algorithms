use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap = BinaryHeap::from(stones);
        while max_heap.len() > 1 {
            let y = max_heap.pop().unwrap();
            let x = max_heap.pop().unwrap();
            if y > x {
                max_heap.push(y - x);
            }
        }
        max_heap.pop().unwrap_or(0)
    }
}
