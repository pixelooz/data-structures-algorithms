use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut max_heap = BinaryHeap::with_capacity(k + 1);

        for point in points {
            let distance = point[0].pow(2) + point[1].pow(2);
            max_heap.push((distance, point));
            if max_heap.len() > k {
                max_heap.pop();
            }
        }
        max_heap.into_iter().map(|(_, point)| point).collect()
    }
}
