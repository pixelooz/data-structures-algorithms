use crate::Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        nums.sort();

        fn explore_subs(
            nums: &[i32],
            index: usize,
            subset: &mut Vec<i32>,
            results: &mut Vec<Vec<i32>>,
        ) {
            results.push(subset.clone());
            for i in index..nums.len() {
                if i > index && nums[i] == nums[i - 1] {
                    continue;
                }
                subset.push(nums[i]);
                explore_subs(nums, i + 1, subset, results);
                subset.pop();
            }
        }
        explore_subs(&nums, 0, &mut vec![], &mut results);
        results
    }
}
