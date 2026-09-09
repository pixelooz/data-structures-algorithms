use crate::Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        candidates.sort();

        fn explore_subs(
            candidates: &[i32],
            index: usize,
            target: i32,
            curr_subset: &mut Vec<i32>,
            sum: i32,
            results: &mut Vec<Vec<i32>>,
        ) {
            if sum == target {
                results.push(curr_subset.clone());
            }
            for i in index..candidates.len() {
                let curr_sum = sum + candidates[i];
                if curr_sum > target {
                    return;
                }
                curr_subset.push(candidates[i]);
                explore_subs(candidates, i, target, curr_subset, curr_sum, results);
                curr_subset.pop();
            }
        }
        explore_subs(&candidates, 0, target, &mut vec![], 0, &mut results);
        results
    }
}
