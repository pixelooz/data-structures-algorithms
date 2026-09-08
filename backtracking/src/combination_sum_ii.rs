use crate::Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        candidates.sort();
        explore_subsets(&candidates, 0, target, &mut vec![], 0, &mut results);
        results.into_iter().collect()
    }
}

fn explore_subsets(
    candidates: &[i32],
    index: usize,
    target: i32,
    path: &mut Vec<i32>,
    curr_sum: i32,
    results: &mut Vec<Vec<i32>>,
) {
    if curr_sum == target {
        results.push(path.clone());
        return;
    }
    for i in index..candidates.len() {
        if i > index && candidates[i] == candidates[i - 1] {
            continue;
        }
        if curr_sum + candidates[i] > target {
            break;
        }
        path.push(candidates[i]);
        explore_subsets(
            candidates,
            i + 1,
            target,
            path,
            curr_sum + candidates[i],
            results,
        );
        path.pop();
    }
}
