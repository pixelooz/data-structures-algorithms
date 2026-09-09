use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        explore_subs(&mut nums, 0, &mut results);
        results
    }
}

fn explore_subs(nums: &mut Vec<i32>, index: usize, results: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        results.push(nums.clone());
        return;
    }
    let mut set = HashSet::new();
    for i in index..nums.len() {
        if set.contains(&nums[i]) {
            continue;
        }
        set.insert(nums[i]);
        nums.swap(i, index);
        explore_subs(nums, index + 1, results);
        nums.swap(i, index);
    }
}

impl Solution {
    pub fn _permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut dup_count = HashMap::new();
        for num in nums.iter() {
            dup_count.entry(*num).and_modify(|e| *e += 1).or_insert(1);
        }
        _explore_subs(nums.len(), &mut dup_count, &mut vec![], &mut results);
        results
    }
}
fn _explore_subs(
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
        _explore_subs(num_size, dup_count, perm, results);
        *dup_count.get_mut(&num).unwrap() += 1;
        perm.pop();
    }
}
