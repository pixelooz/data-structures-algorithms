use std::collections::{BinaryHeap, VecDeque};

use crate::Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0; 26];

        for task in tasks {
            let idx = ((task as u8) - b'A') as usize;
            counts[idx] += 1;
        }
        let mut max_heap = BinaryHeap::with_capacity(26);
        for count in counts {
            if count > 0 {
                max_heap.push(count);
            }
        }
        let mut queue = VecDeque::with_capacity(26);
        let mut time = 0;

        while !max_heap.is_empty() || !queue.is_empty() {
            time += 1;
            if let Some(freq) = max_heap.pop() {
                let remaining_freq = freq - 1;
                if remaining_freq > 0 {
                    queue.push_back((remaining_freq, time + n));
                }
            }
            if let Some(&(freq, ready_time)) = queue.front() {
                if ready_time == time {
                    queue.pop_front();
                    max_heap.push(freq);
                }
            }
        }
        time
    }
}
