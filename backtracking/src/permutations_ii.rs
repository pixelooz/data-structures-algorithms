use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut dup_count = HashMap::new();
        for num in nums.iter() {
            dup_count.entry(*num).and_modify(|e| *e += 1).or_insert(1);
        }
        explore_subs(nums.len(), &mut dup_count, &mut vec![], &mut results);
        results
    }
}

fn explore_subs(
    num_size: usize,
    dup_count: &mut HashMap<i32, i32>,
    perm: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
) {
    if perm.len() == num_size {
        results.push(perm.clone());
        return;
    }
    let nums: Vec<i32> = dup_count.keys().cloned().collect();
    for num in nums {
        let count = dup_count.get_mut(&num).unwrap();
        if *count == 0 {
            continue;
        }
        perm.push(num);
        *dup_count.get_mut(&num).unwrap() -= 1;
        explore_subs(num_size, dup_count, perm, results);
        *dup_count.get_mut(&num).unwrap() += 1;
        perm.pop();
    }
}
