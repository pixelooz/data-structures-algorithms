use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut max_heap = BinaryHeap::with_capacity(3);
        let cap = (a + b + c) as usize;
        let mut result = String::with_capacity(cap);

        if a > 0 {
            max_heap.push((a, b'a'));
        }
        if b > 0 {
            max_heap.push((b, b'b'));
        }
        if c > 0 {
            max_heap.push((c, b'c'));
        }
        while let Some((count1, ch1)) = max_heap.pop() {
            let len = result.len();

            if len >= 2 && result.as_bytes().iter().rev().take(2).all(|&ch| ch == ch1) {
                // Element already exists contiguously, we take the next one and insert
                // and then insert both of them into the heap back again.
                if let Some((count2, ch2)) = max_heap.pop() {
                    result.push(ch2 as char);
                    if count2 > 1 {
                        max_heap.push((count2 - 1, ch2));
                    }
                    max_heap.push((count1, ch1));
                } else {
                    break;
                }
            } else {
                // Element does not exists contiguously so it can be appended to the result.
                result.push(ch1 as char);
                if count1 > 1 {
                    max_heap.push((count1 - 1, ch1));
                }
            }
        }
        result
    }
}
