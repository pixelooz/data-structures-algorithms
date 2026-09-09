use crate::Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        candidates.sort();

        fn explore_subsets(
            candidates: &[i32],
            index: usize,
            target: i32,
            curr_subset: &mut Vec<i32>,
            curr_sum: i32,
            results: &mut Vec<Vec<i32>>,
        ) {
            if curr_sum == target {
                results.push(curr_subset.clone());
                return;
            }
            for i in index..candidates.len() {
                if i > index && candidates[i] == candidates[i - 1] {
                    continue;
                }
                if curr_sum + candidates[i] > target {
                    break;
                }
                curr_subset.push(candidates[i]);
                explore_subsets(
                    candidates,
                    i + 1,
                    target,
                    curr_subset,
                    curr_sum + candidates[i],
                    results,
                );
                curr_subset.pop();
            }
        }
        explore_subsets(&candidates, 0, target, &mut vec![], 0, &mut results);
        results.into_iter().collect()
    }
}
