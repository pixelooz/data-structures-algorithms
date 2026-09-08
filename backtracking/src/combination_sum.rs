use crate::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::explore_subsets(&candidates, 0, &mut vec![], target, &mut result);
        result
    }

    fn explore_subsets(
        candidates: &[i32],
        index: usize,
        curr_sub: &mut Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        if index >= candidates.len() {
            return;
        }
        let curr_sum: i32 = curr_sub.iter().sum();
        if curr_sum > target {
            return;
        }
        if curr_sum == target {
            return result.push(curr_sub.clone());
        }
        curr_sub.push(candidates[index]);
        Self::explore_subsets(candidates, index, curr_sub, target, result);
        curr_sub.pop();
        Self::explore_subsets(candidates, index + 1, curr_sub, target, result);
    }
}
