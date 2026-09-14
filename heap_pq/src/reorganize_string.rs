use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn reorganize_string(strs: String) -> String {
        let mut char_counts = [0; 26];
        let mut max_freq = 0;

        for ch in strs.bytes() {
            let idx = (ch - b'a') as usize;
            char_counts[idx] += 1;
            max_freq = max_freq.max(char_counts[idx]);
        }
        if max_freq > (strs.len() + 1) / 2 {
            return String::new();
        }
        let mut max_heap = BinaryHeap::new();
        for idx in 0..26 {
            if char_counts[idx] > 0 {
                let count = char_counts[idx];
                let ch = idx as u8 + b'a';
                max_heap.push((count, ch));
            }
        }
        let mut prev: Option<(usize, u8)> = None;
        let mut result = String::new();

        while let Some((freq, ch)) = max_heap.pop() {
            result.push(ch as char);

            if let Some(item) = prev {
                max_heap.push(item);
            }
            if freq > 1 {
                prev = Some((freq - 1, ch))
            } else {
                prev = None
            }
        }
        result
    }
}
